use crate::constants::{PLEDAGEPOOLCONTRACT};
use web3::{
    contract::{Options, tokens::{TokenizableItem, Detokenize, Tokenizable}, Error},
    ethabi::Token,
};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct PoolDataInfo {
    pub x1: i32,
    pub x2: i32,
    pub x3: i32,
    pub x4: i32,
    pub x5: i32,
    pub x6: i32,
}

// impl Tokenizable for PoolDataInfo {
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

impl Detokenize for PoolDataInfo {
    fn from_tokens(tokens: Vec<Token>) -> Result<Self, Error>
    where
        Self: Sized {
            println!("tokens: {:?}", tokens);

        Ok( 
            PoolDataInfo {
                x1: 0,
                x2: 0,
                x3: 0,
                x4: 0,
                x5: 0,
                x6: 0,
            }
        )
    }
}

pub async fn search() -> web3::Result<()> {
    let ret = PLEDAGEPOOLCONTRACT.query::<i32, _, _, _>("poolLength", (), None, Options::default(), None).await;
    println!("x : {:?}", ret);

    Ok(())
} 