use crate::contract::api;
use crate::model::pool as ModelPool;
use crate::service::pool as ServicePool;
use actix_web::{http::StatusCode, web, HttpResponse, Responder};

pub async fn search(
    req: ModelPool::SearchRequest,
) -> Result<ModelPool::SearchResponse, StatusCode> {
    // call contract
    let ret = api::search().await;
    println!("search result: {:?}", ret);

    Err(StatusCode::INTERNAL_SERVER_ERROR)
}
