use actix_web::{web, HttpResponse, Responder};

pub async fn login(_req: web::Json<String>) ->  impl Responder {
    // match ServiceEntry::login(req.into_inner()).await { 
    //     Ok(res) => Ok(HttpResponse::Ok().json(ResponseBody::new(SUCCESS, MESSAGE_LOGIN_SUCCESS, res))),
    //     Err(err) => Ok(err.response()),
    // }
    web::Bytes::from_static(b"Hello world!")
}