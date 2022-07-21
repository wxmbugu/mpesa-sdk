use hyper::{body::HttpBody, Client};
use hyper_tls::HttpsConnector;
use std::string::ToString;
use std::{
    io::{stdout, Write},
    str::FromStr,
    //string,
};
use strum_macros::Display;
///! Mpesa to make mpesa transcations
#[derive(Debug)]
pub struct Mpesa {
    consumerkey: String,
    consumersecret: String,
    production_env: Environment,
}
///!  Production Environment
#[derive(Debug, Display)]
pub enum Environment {
    ///!Sandbox app Environment
    #[strum(
        serialize = "https://sandbox.safaricom.co.ke/oauth/v1/generate?grant_type=client_credentials"
    )]
    Sandbox,
    ///! Production app Environment
    #[strum(
        serialize = "https://api.safaricom.co.ke/oauth/v1/generate?grant_type=client_credentials"
    )]
    Production,
}

// const SANDBOXBASEURL: &str =
//     "https://sandbox.safaricom.co.ke/oauth/v1/generate?grant_type=client_credentials";

// const PRODUCTIONBASEURL: &str =
//     "https://api.safaricom.co.ke/oauth/v1/generate?grant_type=client_credentials";
// impl FromStr for Environment {
//     type Err = ();

//     fn from_str(s: &str) -> Result<Environment, ()> {
//         match s {
//             SANDBOXBASEURL => Ok(Environment::Sandbox),
//             PRODUCTIONBASEURL => Ok(Environment::Production),
//             _ => Err(()),
//         }
//     }
// }
impl Mpesa {
    ///! Creates an Mpesa app either sandbox or live app
    pub fn new(key: String, secret: String, env: Environment) -> Mpesa {
        Mpesa {
            consumerkey: key,
            consumersecret: secret,
            production_env: env,
        }
    }
    ///! Returns a token to be used to authenticate a safaricomapp
    ///!Sandbox app or Production app
    //TODO: Set Headers for authentication
    pub async fn get_access_token(&self) -> Result<(), Box<dyn std::error::Error>> {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);
        let uri = self.production_env.to_string().parse()?;
        let mut resp = client.get(uri).await?;
        println!("Response: {}", resp.status());

        while let Some(chunk) = resp.body_mut().data().await {
            stdout().write_all(&chunk?)?;
        }

        Ok(())
        //resp.
    }
}
