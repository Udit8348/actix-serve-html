use actix_files::Files;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Render.com provides a $PORT environment variable
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    println!("Starting server on port {}", port);

    HttpServer::new(|| {
        App::new()
            // Serve files from the "static" directory
            .service(Files::new("/", "./static").index_file("index.html"))
    })
    .bind(("0.0.0.0", port.parse().unwrap()))?
    .run()
    .await
}