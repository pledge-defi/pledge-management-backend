use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchRequest { 
    pub pool_id   : String,
    pub pool_status: String,
    pub page : i32,
    pub page_size: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResponse {
    pub code       : String,
}