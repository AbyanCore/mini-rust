use std::sync::{Arc, Mutex};

use actix::{Actor, Addr, AsyncContext, Handler, Message, StreamHandler};
use actix_web::{get, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use uuid::Uuid;

struct Storage {
    pub uuid: Uuid,
    pub connection: Addr<PublicWs>,
}

impl PartialEq for Storage {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }

    fn ne(&self, other: &Self) -> bool {
        self.uuid != other.uuid
    }
}

type StorageDB = Arc<Mutex<Vec<Storage>>>;

struct PublicWs {
    pub uuid: Uuid,
    pub db: StorageDB,
    pub ip: String,
}

impl Actor for PublicWs {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for PublicWs {
    fn started(&mut self, ctx: &mut Self::Context) {
        self.db.lock().unwrap().push(Storage {
            uuid: self.uuid,
            connection: AsyncContext::address(ctx),
        });

        println!("New Connection Appear : {}", self.ip)
    }

    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => self
                .db
                .lock()
                .unwrap()
                .iter()
                .for_each(|ix| ix.connection.do_send(Broadcast(text.to_string()))),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }

    fn finished(&mut self, ctx: &mut Self::Context) {
        if let Ok(mut db) = self.db.lock() {
            if let Some(id) = db.iter().position(|ix| ix.uuid == self.uuid) {
                db.swap_remove(id);
                println!("Koneksi selesai: {}", self.ip);
            }
        } else {
            eprintln!("Gagal mengunci database");
        }
    }
}

#[get("/ws/")]
async fn index(
    req: HttpRequest,
    stream: web::Payload,
    data: web::Data<StorageDB>,
) -> Result<HttpResponse, Error> {
    let ip = req.peer_addr().unwrap().to_string();
    let resp = ws::start(
        PublicWs {
            uuid: Uuid::new_v4(),
            db: data.get_ref().clone(),
            ip: ip,
        },
        &req,
        stream,
    );

    println!("{:?}", resp);
    resp
}

struct Broadcast(String);

impl Message for Broadcast {
    type Result = ();
}

impl Handler<Broadcast> for PublicWs {
    type Result = ();

    fn handle(&mut self, msg: Broadcast, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db: StorageDB = Arc::new(Mutex::new(Vec::new()));

    HttpServer::new(move || {
        let app_data = web::Data::new(db.clone());
        App::new().app_data(app_data).service(index)
    })
    .bind(("localhost", 9090))?
    .run()
    .await
}
