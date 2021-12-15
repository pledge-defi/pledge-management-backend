use actix_web::web;
use crate::api::{login, search};

pub fn config_services(cfg: &mut web::ServiceConfig) { 
    cfg.service(
        web::scope("/api/v2")
        .service(
            web::resource("/login").route(web::post().to(login))
        )
        .service(
            web::resource("/logout").route(web::post().to(login))
        )
        .service(
            web::resource("/search").route(web::get().to(search))
        )
    );
}