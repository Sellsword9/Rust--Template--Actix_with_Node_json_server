/**
* This is a simple Actix-Web template
*/

// -------------- Crates --------------
use actix_web::{get, web, App, HttpServer, HttpResponse, Responder}; 
use actix_files::Files;
    
/* ENDPOINTS */

/**
 * Calculates the difference between two numbers (Same as subtract)
 */
#[get("/diff/{a}/{b}")] 
async fn diff(path: web::Path<(i32, i32)>) -> impl Responder { 
    let (a, b) = path.into_inner();
    let result: i32 = a - b;
    HttpResponse::Ok().body(format!("±{}", result))
}

#[get("/add/{a}/{b}")]
async fn add(path: web::Path<(i32, i32)>) -> impl Responder {
    let (a, b) = path.into_inner();
    let result: i32 = a + b;
    HttpResponse::Ok().body(result.to_string())
    // Addition is commutative
}

/* ENDPOINTS END */

// -------------- Main --------------
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| { 
        App::new()  
            .service(add) // This line adds 1 endpoint
            .service(diff)// This line adds ANOTHER endpoint
            .service(Files::new("/", "src").index_file("index.html"))
    })
    .bind("127.0.0.1:4321")? // binds to localhost:8080
    .run()
    .await
}
