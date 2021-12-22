use crate::model::user as ModelUser;
use crate::service::user as ServiceUser;
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;

const LOGIN_CODE_SUCCESS: u16 = 200;
const LOGIN_CODE_FAILED: u16 = 400;

const MESSAGE_LOGIN_SUCCESS: &'static str = "登录成功";
const MESSAGE_LOGOUT_SUCCESS: &'static str = "退出成功";
const MESSAGE_LOGIN_FAILED: &'static str = "登录失败";
const MESSAGE_LOGOUT_FAILED: &'static str = "退出失败";

pub async fn login(req: web::Json<ModelUser::LoginRequest>) -> impl Responder {
    println!("zhTian req : {:?}", req);

    match ServiceUser::login(req.into_inner()).await {
        Ok(res) => {
            let body = json!({
            "code": LOGIN_CODE_SUCCESS,
            "message": MESSAGE_LOGIN_SUCCESS,
            "data": {
                "token_id": res.token
            }});
            return HttpResponse::Ok().body(serde_json::to_string(&body).unwrap());
        }
        Err(_err) => {
            let body = json!({
            "code": LOGIN_CODE_FAILED,
            "message": MESSAGE_LOGOUT_FAILED,
            });
            return HttpResponse::Ok().body(serde_json::to_string(&body).unwrap());
        }
    }
}

pub async fn logout(logout_req: web::Json<ModelUser::LogoutRequest>) -> impl Responder {
    match ServiceUser::logout(logout_req.into_inner()).await {
        Ok(_res) => {
            let body = json!({
                "code": LOGIN_CODE_SUCCESS,
                "message": MESSAGE_LOGOUT_SUCCESS,
            });
            return HttpResponse::Ok().body(serde_json::to_string(&body).unwrap());
        }
        Err(_err) => {
            let body = json!({
            "code": LOGIN_CODE_FAILED,
            "message": MESSAGE_LOGIN_FAILED,
            });
            return HttpResponse::Ok().body(serde_json::to_string(&body).unwrap());
        }
    }
}
