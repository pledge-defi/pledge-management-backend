use crate::model::{user as ModelUser};
use crate::service::{user as ServiceUser};
use actix_web::{web, HttpResponse, Responder};

pub async fn login(req: web::Json<ModelUser::LoginRequest>) -> impl Responder {
    println!("zhTian req : {:?}", req);

    match ServiceUser::login(req.into_inner()).await {
        Ok(res) => HttpResponse::Ok().body(serde_json::to_string(&res).unwrap()),
        Err(_) => HttpResponse::Ok().body("Login Failed"),
    }
}

pub async fn logout(logout_req: web::Json<ModelUser::LogoutRequest>) -> impl Responder {
    match ServiceUser::logout(logout_req.into_inner()).await {
		Ok(res)  => HttpResponse::Ok().body(serde_json::to_string(&res).unwrap()),
        Err(_) => HttpResponse::Ok().body("Logout Failed"),
	}
}

