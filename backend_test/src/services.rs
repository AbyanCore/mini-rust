use actix_web::{
    get, post, web::{Data,Json, Path}, HttpResponse, Responder
};
use serde::Deserialize;
use crate::{
    messages::{FetchUser},
    AppState,DbActor
};
use actix::Addr;

#[get("/")]
pub async fn landing() -> impl Responder {
    HttpResponse::Ok().json("Hello World")
}

#[get("/users")]
pub async fn fetch_users(state: Data<AppState>) -> impl Responder {
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(FetchUser).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json("No Users Found"),
        _ => HttpResponse::InternalServerError().json("Unable to retrieve users"),
    }
}