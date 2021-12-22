use crate::constants::ADMINDBPOOL;
use crate::schema::user::{admin::dsl::*, Admin};
use actix_web::http::StatusCode;
use bcrypt::verify;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginRequest {
    pub name: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserInfo {
    pub user_id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenBodyResponse {
    pub token: String,
    pub token_type: String,
    pub user: UserInfo,
}

// login
pub async fn login(login_req: LoginRequest) -> Result<UserInfo, StatusCode> {
    println!("zhTian login req : {:?}", login_req);

    let conn = &ADMINDBPOOL.get().unwrap();

    println!("==== admin : {:?}", admin);

    match admin
        .filter(name.eq(&login_req.name))
        .get_result::<Admin>(conn)
    {
        Ok(user_to_verify) => {
            println!("user to verify : {:?}", user_to_verify);
            println!("login password: {}", login_req.password);
            println!("verify password: {}", user_to_verify.password);
            println!(
                "verify result : {}",
                verify(&login_req.password, &user_to_verify.password).unwrap()
            );
            if !user_to_verify.password.is_empty()
                && verify(&login_req.password, &user_to_verify.password).unwrap()
            {
                let admin_info = UserInfo {
                    user_id: user_to_verify.user_id,
                    name: user_to_verify.name.clone(),
                };

                return Ok(admin_info);
            } else {
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        }

        Err(e) => {
            println!("e : {:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogoutRequest {
    pub token_id: String,
    // pub token_type: String,
}
//pub async fn logout(_logout_req: LogoutRequest) -> Result<String, StatusCode> {
pub async fn logout() -> Result<String, StatusCode> {
    // UserToken::disable_token(&logout_req.token);
    // Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, MESSAGE_LOGIN_FAILED.to_string()))

    Ok("Logout success".to_string())
}
