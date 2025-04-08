mod compression;
mod database;
use compression::compress_file;
use database::update_compressed_file;
use sqlx::Row;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    let mut file_ids: Vec<i64> = Vec::new();
    let mut comp_lev: Vec<u32> = Vec::new();

    // Loop for user input to get the file ID and compression level
    loop {
        let mut response = String::new();
        let mut file_id_input = String::new();
        let mut comp_level_input = String::new();

        println!("Do you want to compress a file (y/n)");

        std::io::stdin()
            .read_line(&mut response)
            .expect("Failed to take response");

        if response.trim() == "n" {
            break;
        } else if response.trim() == "y" {
            println!("Enter the file ID to be compressed");
            std::io::stdin()
                .read_line(&mut file_id_input)
                .expect("Failed to take file ID");

            let file_id: i64 = file_id_input.trim().parse().unwrap();
            file_ids.push(file_id);

            println!("Enter the compression level (0-9):");
            std::io::stdin()
                .read_line(&mut comp_level_input)
                .expect("Failed to read compression level");

            let level: u32 = comp_level_input.trim().parse().unwrap_or(6); // Default to level 6 if invalid
            comp_lev.push(level);
        } else {
            println!("Invalid input, please enter 'y' or 'n'");
        }
    }

    // Setup the database connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgresql://ghis:password@10.153.115.29:5432/filesDB")
        .await
        .unwrap();

    // Loop through file IDs and compress files
    for (i, &file_id) in file_ids.iter().enumerate() {
        let level = comp_lev[i];

        // Query the database to fetch the file name/path using the file ID
        let query = "SELECT file_name FROM files WHERE id = $1 LIMIT 1";
        let result = sqlx::query(query).bind(file_id).fetch_one(&pool).await;

        match result {
            Ok(row) => {
                let file_path: String = row.get("file_name"); // Get the file path from the database

                // Compress the file (compress_file should return a Result)
                match compress_file(&file_path, level) {
                    Ok(compressed_file_path) => {
                        // Update the file status to 'compressing' in the database
                        sqlx::query("UPDATE files SET status = 'compressing' WHERE id = $1")
                            .bind(file_id)
                            .execute(&pool)
                            .await
                            .unwrap();

                        // After the file is compressed, update the status to 'compressed'
                        let status_query = format!(
                            "UPDATE files SET status = 'compressed' WHERE id = {}",
                            file_id
                        );
                        sqlx::query(&status_query).execute(&pool).await.unwrap();

                        // Optionally, store the compressed file path in the database (if necessary)
                        update_compressed_file(&pool, file_id, &compressed_file_path)
                            .await
                            .unwrap();

                        println!("File {} compressed successfully", i + 1);
                    }
                    Err(error) => {
                        eprint!("Error compressing file {}: {}", i + 1, error);
                    }
                }
            }
            Err(error) => {
                eprint!(
                    "Error finding file in database with ID {}: {}",
                    file_id, error
                );
            }
        }
    }
}
