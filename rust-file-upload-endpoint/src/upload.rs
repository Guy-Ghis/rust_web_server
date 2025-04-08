use axum::{Extension, extract::Multipart, http::StatusCode, response::IntoResponse};
use sqlx::{PgPool, Row};
use std::{fs::File, io::Write, path::Path};

pub async fn upload(
    Extension(pool): Extension<PgPool>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, StatusCode> {
    while let Some(field) = multipart.next_field().await.ok().flatten() {
        if field.name().unwrap_or("") != "fileupload" {
            continue;
        }

        // Get file name
        if let Some(file_name) = field.file_name().map(String::from) {
            let file_path = format!("../uploads/{}", file_name);

            // Ensure the directory exists
            let path = Path::new("../uploads");
            if !path.exists() {
                std::fs::create_dir_all(path).expect("Failed to create 'uploads' directory!");
            }
            println!("File path: {}", file_path);

            // Read file bytes and save the file
            if let Ok(data) = field.bytes().await {
                if let Ok(mut file_handle) = File::create(&file_path) {
                    if file_handle.write_all(&data).is_ok() {
                        // Successfully saved the file
                        println!("File '{}' uploaded successfully!", file_name);
                    }
                }
            }

            // Insert file info into the database
            match insert_into_db(&pool, &file_name, "uploaded").await {
                Ok(file_id) => {
                    println!("File ID inserted into DB: {}", file_id);
                }
                Err(e) => {
                    eprintln!("Failed to insert file into the Database: {}", e);
                    return Err(StatusCode::INTERNAL_SERVER_ERROR);
                }
            }
        }
    }
    Ok(())
}

// Function to insert file details into the database and return the inserted ID
async fn insert_into_db(pool: &PgPool, file_name: &str, status: &str) -> Result<i32, sqlx::Error> {
    println!("Inserting into DB: '{}' -> '{}'", file_name, status);

    let row =
        sqlx::query("INSERT INTO files (file_name, status) VALUES ($1, $2::status) RETURNING id")
            .bind(file_name)
            .bind(status)
            .fetch_one(pool)
            .await?;

    let file_id: i32 = row.get("id");
    println!("Insert successful! File ID: {}", file_id);

    Ok(file_id)
}
