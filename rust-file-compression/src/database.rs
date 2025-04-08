pub async fn update_compressed_file(
    pool: &sqlx::PgPool,
    file_id: i64,
    compressed_file: &str,
) -> Result<(), sqlx::Error> {
    let query = "UPDATE files SET compressed_file = $1 WHERE id = $2";

    sqlx::query(query)
        .bind(compressed_file) // Bind the compressed file path
        .bind(file_id) // Bind the file ID
        .execute(pool)
        .await?;

    Ok(())
}
