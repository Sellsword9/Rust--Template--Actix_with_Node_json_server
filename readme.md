# Rust Actix-based Server Template

This is a simple template for creating Rust Actix-based servers that utilize Node and Json-server for storing data.

## Prerequisites

Before getting started, make sure you have the following installed:

- Rust: [Installation Guide](https://www.rust-lang.org/tools/install)
- Node.js: [Installation Guide](https://nodejs.org/en/download/)

## Getting Started

1. Clone this repository:

  ```bash
  git clone https://github.com/sellsword9/rust-js-json-server-node--template.git
  ```

2. Install the required dependencies for the Rust and npm:

  ```bash
  cd rust-js-json-server-node--template/src && npm i && cargo build
  ```

3. Start the Json-server and Actix:

  ```bash
  npm run serve & cargo r
  ```
## Usage
```Rust 
// src/main.rs
// Make your functions something like this in Rust
#[get("/diff/{a}/{b}")] 
async fn diff(path: web::Path<(i32, i32)>) -> impl Responder {  // This line names an endpoint
    let (a, b) = path.into_inner();
    let result: i32 = a - b;
    HttpResponse::Ok().body(format!("±{}", result))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| { 
        App::new()  
            .service(diff)// This line adds an endpoint
            .service(Files::new("/", "src").index_file("index.html"))
    })
    .bind("127.0.0.1:4321")?
```
and
```JavaScript
// src/main.js
// Do what you need in js
```

## Contributing

Contributions are welcome! If you find any issues or have suggestions for improvements, please open an issue or submit a pull request.

## License
None, this is CC0, completely free to use for any purpose with/without attribution. No warranties.
