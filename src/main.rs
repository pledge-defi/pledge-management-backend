#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

extern crate chrono;

mod api;
mod config;
mod constants;
mod model;
mod schema;
mod service;
mod auth;

use actix_web::{App, HttpServer};
use std::env;
use actix_web_httpauth::middleware::HttpAuthentication;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().expect("Failed to read .env file");
    env::set_var("RUST_LOG", "actix_web=info,actix_server=info,debug");

    let app_host = env::var("APP_HOST").expect("APP_HOST not found.");
    let app_port = env::var("APP_PORT").expect("APP_PORT not found.");
    let app_url = format!("{}:{}", &app_host, &app_port);
    println!("url: {}", app_url);

    let auth = HttpAuthentication::bearer(auth.bearer_auth_validator);
    HttpServer::new(|| {
        App::new()
            .wrap(auth)
            .wrap(actix_web::middleware::Logger::default())
            .configure(config::config_services)
    })
    .bind(&app_url)?
    .run()
    .await
}
