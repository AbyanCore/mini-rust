mod models;
use std::time::SystemTime;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use rand::Rng;

#[actix_web::get("/greet")]
async fn greet() -> impl Responder {
    format!("Hello Actix")
}

#[actix_web::get("/status")]
async fn status() -> impl Responder {
    HttpResponse::Ok().json(models::Status {
        status: "Up",
        code: 200
    })
}

#[actix_web::get("/random")]
async fn random() -> impl Responder {
    HttpResponse::Ok().json(models::Response {
        data: &format!("{}",rand::thread_rng().gen_range(0..100)).to_string(),
        message: "Success",
        code: 200
    })
} 

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = 8080;

    println!("Starting Server On 127.0.0.1:{}",port);
    
    HttpServer::new(move || {
        App::new().service(greet).service(status).service(random)
    }).bind(("127.0.0.1",port))?
    .workers(2)
    .run().await
}
