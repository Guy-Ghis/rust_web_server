mod compression;
mod database;
use compression::compress_file;
use database::insert_user;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    let mut input_files: Vec<String> = Vec::new();
    let mut output_files: Vec<String> = Vec::new();
    let mut comp_meth: Vec<String> = Vec::new();
    loop {
        let mut response = String::new();
        let mut i_file = String::new();
        let mut o_file = String::new();
        let mut comp_method = String::new();
        println!("Do you want to compress files (y/n)");

        std::io::stdin()
            .read_line(&mut response)
            .expect("Failed to take response");

        if response.trim() == "n" {
            break;
        } else if response.trim() == "y" {
            println!("Enter the file to be compressed");
            std::io::stdin()
                .read_line(&mut i_file)
                .expect("Failed to take file");
            input_files.push(i_file.trim().to_string());

            println!("Enter path to store compressed file");
            std::io::stdin()
                .read_line(&mut o_file)
                .expect("Failed to take output path");
            output_files.push(o_file.trim().to_string());

            println!("Enter compression method");
            std::io::stdin()
                .read_line(&mut comp_method)
                .expect("Failed to take compression method");
            comp_meth.push(comp_method.trim().to_string());
        }
    }
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgresql://ghis:password@10.153.115.29:5432/filesDB")
        .await
        .unwrap();

    for i in 0..output_files.len() {
        let (input, output, method) = (&input_files[i], &output_files[i], &comp_meth[i]);

        let result = compress_file(input, output, method);

        let response_id = insert_user(&pool, &pool, &input, &output).await;

        match result {
            Ok(_) => {
                
                sqlx::query("UPDATE files SET status = 'completed' WHERE id = $1")
                    .bind(response_id)
                    .execute(&pool)
                    .await
                    .unwrap();
                let status = format!("UPDATE files SET status = 'completed' WHERE id = {}", response_id);

                let status_result = sqlx::query(&status).execute(&pool).await.unwrap();
                
                println!("STATUS: {:?}", status_result);
                
                println!("The file number {} was compressed successfully", i + 1);
            }
            Err(error) => {
                eprint!("Error: {}", error)
            }
        }
    }
}
