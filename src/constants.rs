use diesel::{
    mysql::MysqlConnection,
    r2d2::{self, ConnectionManager},
};
use std::env;

pub type Connection = MysqlConnection;
pub type Pool = r2d2::Pool<ConnectionManager<Connection>>;

// pledge database
lazy_static! {
    pub static ref ADMINDBPOOL: Pool = {
        let db_url = env::var("PLEDAGE_DATABASE_URL").expect("PLEDAGE_DATABASE_URL not found.");
        let manager = ConnectionManager::<Connection>::new(db_url);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");

        pool
    };
}
