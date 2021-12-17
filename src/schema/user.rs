table! {
    admin (user_id) {
        user_id     -> Int4,
        name        -> Varchar,
        password    -> Varchar,
    }
}

#[derive(Queryable, QueryableByName, Insertable, Serialize, Deserialize, Debug)]
#[table_name = "admin"]
pub struct Admin {
    pub user_id: i32,
    pub name: String,
    pub password: String,
}
