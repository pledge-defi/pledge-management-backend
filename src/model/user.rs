use diesel::prelude::*;
use bcrypt::verify;
use crate::constants::ADMINDBPOOL;
use crate::schema::user::{
    admin::dsl::*, Admin
};
use actix_web::http::StatusCode;
use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginRequest { 
    pub name   : String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserInfo {
    pub user_id : i32,
    pub name    : String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenBodyResponse {
    pub token       : String,
    pub token_type  : String,
    pub user        : UserInfo,
}

// login
pub async fn login(login_req: LoginRequest) -> Result<UserInfo, StatusCode> {
    let conn = &ADMINDBPOOL.get().unwrap();

    if let Ok(user_to_verify) = admin.filter(name.eq(&login_req.name)).get_result::<Admin>(conn) {   
        if !user_to_verify.password.is_empty() &&
            verify(&login_req.password, &user_to_verify.password).unwrap() {
                let admin_info = UserInfo {
                    user_id : user_to_verify.user_id,
                    name    : user_to_verify.name.clone(),
                };

            return Ok(admin_info);
        }
    }

    Err(StatusCode::INTERNAL_SERVER_ERROR)
}
