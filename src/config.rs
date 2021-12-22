use crate::api::{login, logout};
use actix_web::web;

pub fn config_services(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v2")
            .service(web::resource("/user/login").route(web::post().to(login)))
            .service(web::resource("/user/logout").route(web::post().to(logout))),
    );
}
