use actix_web::{web, Responder, HttpResponse, http::StatusCode};
use crate::service::{pool as ServicePool};
use crate::model::{pool as ModelPool};
use crate::contract::api;

pub async fn search(req: ModelPool::SearchRequest) -> Result<ModelPool::SearchResponse, StatusCode> {
    // call contract
    let ret = api::search().await;
    println!("search result: {:?}", ret);

    Err(StatusCode::INTERNAL_SERVER_ERROR)
}

