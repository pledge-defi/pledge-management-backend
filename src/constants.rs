use diesel::{
    mysql::MysqlConnection,
    r2d2::{self, ConnectionManager},
};
use std::env;
use std::str::FromStr;
use web3::{
    self,
    contract::{Contract, Options},
    types::Address,
    Transport, Web3,
};

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

// BSC testnet rpc
static BSC_RPC: &'static str = "https://data-seed-prebsc-1-s1.binance.org:8545";
lazy_static! {
    pub static ref WEB3: web3::Web3<web3::transports::http::Http> = {
        let transport = web3::transports::Http::new(BSC_RPC).unwrap();
        let web3 = web3::Web3::new(transport);

        web3
    };
}

static PLEDGE_POOL_ADDRESS: &'static str = "0x08A5125C84C3DAb4834A28e73A35F4b6d895E7AA";
// pledge pool contract
lazy_static! {
    pub static ref PLEDAGEPOOLCONTRACT: web3::contract::Contract<web3::transports::http::Http> = {
        let address = Address::from_str(PLEDGE_POOL_ADDRESS).unwrap();
        let contract = Contract::from_json(
            WEB3.eth(),
            address,
            include_bytes!("./contract/res/PledgePool.json"),
        )
        .unwrap();

        contract
    };
}
