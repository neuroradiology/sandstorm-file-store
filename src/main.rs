use actix_web::{web, App, HttpResponse, HttpServer, Responder, Error};
use std::fs::File;
use std::io::Write;
use std::path::Path;

async fn handle_get() -> impl Responder {
    println!("GET request received");
    HttpResponse::Ok().body("GET request processed")
}

async fn handle_post(body: String) -> Result<impl Responder, Error> {
    println!("POST request received with data: {}", body);
    Ok(HttpResponse::Ok().body(format!("POST request processed with data: {}", body)))
}

async fn handle_put(body: String) -> Result<impl Responder, Error> {
    println!("PUT request received with data: {}", body);

    let file_path = Path::new("/var/data/data.txt");

    // Attempt to open the file in append mode
    let mut file = match File::create(&file_path) {
        Ok(file) => {
            println!("Successfully opened file for writing.");
            file
        },
        Err(e) => {
            println!("Failed to open file: {}", e);
            return Ok(HttpResponse::InternalServerError().body("Failed to open file"));
        },
    };

    // Attempt to write the body to the file
    if let Err(e) = writeln!(file, "{}", body) {
        println!("Failed to write to file: {}", e);
        return Ok(HttpResponse::InternalServerError().body("Failed to write to file"));
    }

    // Log success and return a response
    println!("PUT request processed successfully with data: {}", body);
    Ok(HttpResponse::Ok().body(format!("PUT request processed with data: {}", body)))
}

async fn handle_delete() -> impl Responder {
    println!("DELETE request received");
    HttpResponse::Ok().body("DELETE request processed")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .route("/", web::get().to(handle_get))       // Handle GET requests
        .route("/", web::post().to(handle_post))     // Handle POST requests
        .route("/", web::put().to(handle_put))       // Handle PUT requests
        .route("/", web::delete().to(handle_delete)) // Handle DELETE requests
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
