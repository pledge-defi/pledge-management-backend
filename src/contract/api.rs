use crate::constants::{PLEDAGEPOOLCONTRACT, WEB3};
use web3::{
    contract::{Options, tokens::{TokenizableItem, Detokenize, Tokenizable}, Error},
    ethabi::Token,
    types::{U256,CallRequest}
};
use serde::{Serialize, Deserialize};
use std::vec::Vec;

#[derive(Serialize, Deserialize, Debug)]
struct PoolDataInfo {
    pub x1: U256,
    pub x2: U256,
    pub x3: U256,
    pub x4: U256,
    pub x5: U256,
    pub x6: U256,
}

// impl Tokenizable for std::vec::Vec<PoolDataInfo> {
//     fn from_token(token: Token) -> Result<Self, Error>
//     where
//         Self: Sized {
//             let x = token.into_array().unwrap();
//             println!("x = {:?}", x);

//             Ok(
//                 vec![PoolDataInfo {
//                     x1: 0,
//                     x2: 0,
//                     x3: 0,
//                     x4: 0,
//                     x5: 0,
//                     x6: 0,
//                 }]
//             )
//     }

//     fn into_token(self) -> Token {
//         Token::Array(vec![])
//     }
// }

// impl TokenizableItem for PoolDataInfo {
//     fn from_token(token: Token) -> Result<Self, Error>
//     where
//         Self: Sized {
//             let x = token.into_array().unwrap();
//             println!("x = {:?}", x);

//             Ok(
//                 PoolDataInfo {
//                     x1: 0,
//                     x2: 0,
//                     x3: 0,
//                     x4: 0,
//                     x5: 0,
//                     x6: 0,
//                 }
//             )
//     }

//     fn into_token(self) -> Token {
//         Token::Array(vec![])
//     }
// }

// impl Detokenize for PoolDataInfo {
//     fn from_tokens(tokens: Vec<Token>) -> Result<Self, Error>
//     where
//         Self: Sized {
        
//             println!("tokens: {:?}", tokens);

//         Ok( 
//             PoolDataInfo {
//                 x1: U256::zero(),
//                 x2: U256::zero(),
//                 x3: U256::zero(),
//                 x4: U256::zero(),
//                 x5: U256::zero(),
//                 x6: U256::zero(),
//             }
//         )
//     }
// }

// impl TokenizableItem for Vec<PoolDataInfo> {}

// impl Detokenize for Vec<PoolDataInfo> {
//     fn from_tokens(tokens: Vec<Token>) -> Result<Self, Error>
//     where
//         Self: Sized {
        
//             println!("tokens: {:?}", tokens);

//         Ok( 
//             vec![PoolDataInfo {
//                 x1: U256::zero(),
//                 x2: U256::zero(),
//                 x3: U256::zero(),
//                 x4: U256::zero(),
//                 x5: U256::zero(),
//                 x6: U256::zero(),
//             }]
//         )
//     } 
// }

pub async fn search() -> web3::Result<()> {
    println!("PLEDAGEPOOLCONTRACT.address() = {:?}", PLEDAGEPOOLCONTRACT.address());

    let pool_length = PLEDAGEPOOLCONTRACT.query::<i32, _, _, _>("poolLength", (), None, Options::default(), None).await.unwrap_or(0_i32);
    println!("poolLength : {:?}", pool_length);
    for index in 0..pool_length {
        let pool_data_info = PLEDAGEPOOLCONTRACT.query::<i32, _, _, _>("getPoolState", (index), None, Options::default(), None).await;
        // let pool_data_info = PLEDAGEPOOLCONTRACT.call("poolDataInfo", index, PLEDAGEPOOLCONTRACT.address(), Options::default()).await.expect("xxx---");
        println!("pool_data_info_{} : {:#?}", index, pool_data_info);
    }

    // let lend_fee = PLEDAGEPOOLCONTRACT.query::<U256, _, _, _>("lendFee", (), None, Options::default(), None).await;
    // println!("lendFee : {:?}", lend_fee);

    Ok(())
} 