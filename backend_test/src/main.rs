use actix::SyncArbiter;
use actix_web::{web::Data,App,HttpServer};
use dotenv::dotenv;
use diesel::{
    r2d2::{ConnectionManager,Pool},
    PgConnection
};
use std::env;

mod db_models;
mod db_utils;
mod schema;
mod services;
mod messages;
mod actors;

use db_utils::{get_pool,AppState,DbActor};
use services::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool: Pool<ConnectionManager<PgConnection>> = get_pool(&db_url);
    let db_addr = SyncArbiter::start(5, move || DbActor(pool.clone()));

    HttpServer::new(move || {
        App::new()
        .app_data(Data::new(AppState  { db: db_addr.clone() } ))
        .service(fetch_users)
        .service(landing)
    }).bind(("192.168.88.54",8080))?.run().await    
}
