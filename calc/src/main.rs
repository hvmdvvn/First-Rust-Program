use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};
use calc::{add, subtract, multiply, divide}; // 👈 import from lib.rs

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the Wem calculator service!")
}

#[get("/add/{a}/{b}")]
async fn add_route(path: web::Path<(i32, i32)>) -> impl Responder {
    let (a, b) = path.into_inner();
    HttpResponse::Ok().body(add(a, b).to_string())
}

#[get("/subtract/{a}/{b}")]
async fn subtract_route(path: web::Path<(i32, i32)>) -> impl Responder {
    let (a, b) = path.into_inner();
    HttpResponse::Ok().body(subtract(a, b).to_string())
}

#[get("/multiply/{a}/{b}")]
async fn multiply_route(path: web::Path<(i32, i32)>) -> impl Responder {
    let (a, b) = path.into_inner();
    HttpResponse::Ok().body(multiply(a, b).to_string())
}

#[get("/divide/{a}/{b}")]
async fn divide_route(path: web::Path<(i32, i32)>) -> impl Responder {
    let (a, b) = path.into_inner();
    match divide(a, b) {
        Ok(result) => HttpResponse::Ok().body(result.to_string()),
        Err(e) => HttpResponse::BadRequest().body(e),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(add_route)
            .service(subtract_route)
            .service(multiply_route)
            .service(divide_route)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
