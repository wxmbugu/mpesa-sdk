extern crate serde;
use std::error::Error;

use serde::{Deserialize, Serialize};
use strum_macros::Display;
use thiserror::Error;
/// Mpesa to make mpesa transcations
#[derive(Debug)]
pub struct Mpesa {
    consumerkey: String,
    consumersecret: String,
    pub production_env: Environment,
}
///  Production Environment
#[derive(Debug, Serialize, Display)]
pub enum Environment {
    ///!Sandbox app Environment
    #[strum(serialize = "https://sandbox.safaricom.co.ke")]
    Sandbox,
    ///! Production app Environment
    #[strum(serialize = "https://api.safaricom.co.ke")]
    Production,
}

#[derive(Error, Debug)]
pub enum MpesaErrors {
    #[error("Invaid authentication")]
    BadCredentials,
}

///Mpesa access_token response
#[derive(Deserialize, Debug)]
pub struct AccessToken {
    pub expires_in: String,
    pub access_token: String,
}

impl Mpesa {
    ///! Creates an Mpesa app can be either on a sandbox or prodcution Environment
    pub fn new(key: String, secret: String, env: Environment) -> Mpesa {
        Mpesa {
            consumerkey: key,
            consumersecret: secret,
            production_env: env,
        }
    }
    /// Returns a token to be used to authenticate a safaricom app
    /// Sandbox app or Production app
    /// Sets a basic_auth to get access_token
    pub async fn get_access_token(&self) -> Result<AccessToken, Box<dyn Error>> {
        let client = reqwest::Client::new();
        let resp = client
            .get(format!(
                "{}/oauth/v1/generate?grant_type=client_credentials",
                self.production_env
            ))
            .basic_auth(&self.consumerkey, Some(&self.consumersecret))
            .send()
            .await?;
        //println!("{:?}", resp);

        match resp.status().as_str() {
            "200" => return Ok(resp.json::<AccessToken>().await?),
            _ => Err(Box::new(MpesaErrors::BadCredentials)),
        }
    }
}
// pub struct Data<T: Serialize, U: for<'a> Deserialize<'a>> {
//     requestdata: T,
//     responsedata: U,
//     access_token: String,
//     env: Environment,
// }
//             .json(&self.requestdata)
//             .send()
//             .await?;
//         println!("{:#?}", resp);
//         Ok(())
//     }
//     pub async fn getrequest(&self) -> Result<(), Box<dyn Error>> {
//         let client = reqwest::Client::new();
//         let resp = client
//             .post(format!(
//                 "{}/oauth/v1/generate?grant_type=client_credentials",
//                 self.env.to_string()
//             ))
//             .json(&self.requestdata)
//             .send()
//             .await?;
//         println!("{:#?}", resp);
//         Ok(())
//     }
// }

//pub trait Transcations<T> {
//    fn send(&self) -> Option<T>;
//}
// #[derive(Serialize)]
// pub struct Data<T>;
// pub async fn postrequest(uri: String, data: Data<T>) -> Result<(), Box<dyn Error>> {
//     let client = reqwest::Client::new();
//     let resp = client
//         .post(format!(
//             "{}/oauth/v1/generate?grant_type=client_credentials",
//             uri
//         ))
//         .json(&data)
//         .send()
//         .await?;
//     println!("{:#?}", resp);
//     Ok(())
// }
