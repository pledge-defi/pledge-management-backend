use actix_web::{web, Responder, HttpResponse};
use crate::service::{user as ServiceUser, pool as ServicePool};
use crate::model::{user as ModelUser, pool as ModelPool};

pub async fn login(req: web::Json<ModelUser::LoginRequest>) -> impl Responder {
    match ServiceUser::login(req.into_inner()).await { 
        Ok(res) => HttpResponse::Ok().body(serde_json::to_string(&res).unwrap()),
        Err(_) => HttpResponse::Ok().body("Login Failed"),
    }
}

pub async fn search(req: web::Json<ModelPool::SearchRequest>) -> impl Responder {
    // call contract.
    match ServicePool::search(req.into_inner()).await {
        Ok(res) => HttpResponse::Ok().body(serde_json::to_string(&res).unwrap()),
        Err(_) => HttpResponse::Ok().body("seach pool Failed"),
    }
}