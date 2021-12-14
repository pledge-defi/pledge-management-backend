use actix_web::{    
    http::{
        StatusCode,
    }
};

use crate::model::user as ModelUser;
use serde_json::json;
use crate::model::user_token::UserToken;

pub async fn login(login_req: ModelUser::LoginRequest) -> Result<ModelUser::TokenBodyResponse, StatusCode> {
    match ModelUser::login(login_req).await {
        Ok(logged_user) => {
            match serde_json::from_value(json!({ "token": UserToken::generate_token(&logged_user).await, "token_type": "api_key", "user": logged_user })) {
                Ok(res) => Ok(res),
                Err(e) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        },
        Err(_err) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}
