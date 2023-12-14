use actix_web::{get, web, App, HttpServer, HttpResponse, Responder}; 
use actix_files::Files;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| { 
        App::new()  
            .service(Files::new("/", "src").index_file("index.html"))
    })
    .bind("127.0.0.1:4321")? // binds to localhost:8080
    .run()
    .await
}
