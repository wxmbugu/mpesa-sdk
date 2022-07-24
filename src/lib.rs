//use hyper::header;
//use hyper::Client;
//use hyper::Request;
//use hyper_tls::HttpsConnector;
//use std::io::{stdout, Write};
extern crate serde;
use serde::Deserialize;
use std::string::ToString;
use strum_macros::Display;
/// Mpesa to make mpesa transcations
#[derive(Debug)]
pub struct Mpesa {
    consumerkey: String,
    consumersecret: String,
    production_env: Environment,
}
///  Production Environment
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

#[derive(Deserialize, Debug)]
struct AccessResponse {
    expires_in: String,
    access_token: String,
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
    /// Returns a token to be used to authenticate a safaricomapp
    /// Sandbox app or Production app
    /// Sets a basic_auth to get access_token
    pub async fn get_access_token(&self) -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let resp = client
            .get(self.production_env.to_string())
            .basic_auth(&self.consumerkey, Some(&self.consumersecret))
            .send()
            .await?;
        //.basic_auth(&self.consumerkey, Some(&self.consumersecret))
        //.await?;
        //print!("{:?}", resp);
        println!("{:#?}", resp);
        let resp_json = resp.json::<AccessResponse>().await;
        println!("{:#?}", resp_json);
        Ok(())
        //resp.
    }
}
