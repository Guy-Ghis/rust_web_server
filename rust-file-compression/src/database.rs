pub async fn insert_user(
    pool1: &sqlx::PgPool,
    pool2: &sqlx::PgPool,
    file_name: &str,
    compressed_file: &str,
) -> i64 {
    // Blocking database operation inside an async function (inefficient)
    let query = "INSERT INTO files (file_name, compressed_file) VALUES ($1, $2)";
    sqlx::query(query)
        .bind(file_name)
        .bind(compressed_file)
        .execute(pool1)
        .await
        .unwrap();
    let id = "SELECT id FROM files WHERE file_name=$1";

    let result = sqlx::query(id)
        .bind(file_name)
        .execute(pool2)
        .await
        .unwrap();
    let id = result.rows_affected();
    println!("task_id: {}", id);
    id as i64
}