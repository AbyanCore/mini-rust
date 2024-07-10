use std::{fmt::format, sync::{Arc, Mutex}};

use actix_web::{ web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,Debug)]
struct StorageIp{
    id: u32,
    ip: String
}

#[derive(Serialize,Deserialize,Debug)]
struct DefaultMessage {
    ip: String,
    message: String,
    status: bool
}

#[actix_web::get("/")]
async fn landing(req: HttpRequest,db: web::Data<IpDB>) -> impl Responder {
    let mut data = db.lock().unwrap();
    let new_id = data.len() as u32;
    data.push(StorageIp {
        id: new_id,
        ip: req.peer_addr().unwrap().to_string()
    });

    HttpResponse::Ok().json(DefaultMessage {
        ip: req.peer_addr().unwrap().to_string(),
        message: format!("Connected.\n Sum Connection : {} ", data.len()),
        status: true
    })
}

#[actix_web::get("/list-connection")]
async fn list_connection(req: HttpRequest,db: web::Data<IpDB>) -> impl Responder {
    let data = db.lock().unwrap();
    let mut printed: String = "".to_string();

    for ix in data.iter() {
        printed += &format!("{:?} \n.", ix);
    }
    
    HttpResponse::Ok().json(DefaultMessage {
        ip: req.peer_addr().unwrap().to_string(),
        message: printed,   
        status: true
    })
}

#[actix_web::get("/add/{a}/{b}")]
async fn adder(path: web::Path<(i32,i32)>,req: HttpRequest) -> impl Responder {
    let (a,b) = path.into_inner();

    HttpResponse::Ok().json(DefaultMessage {
        ip: req.peer_addr().unwrap().to_string(),
        message: format!("Is {}",a + b),   
        status: true
    })
}

type IpDB = Arc<Mutex<Vec<StorageIp>>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let IpData: IpDB = Arc::new(Mutex::new(Vec::new()));

    HttpServer::new(move || {
        let app_data = web::Data::new(IpData.clone());
        App::new().app_data(app_data).service(landing).service(adder).service(list_connection)
    }).bind(("192.168.88.42",8080))?.run().await
}
