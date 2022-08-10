#![allow(dead_code)]
extern crate serde;
//use reqwest::Client;
use reqwest::Client;
use std::error::Error;
//use services::lipanampesa;
//use reqwest::Client;
use serde::{Deserialize, Serialize};
use strum_macros::Display;
use thiserror::Error;
/// Mpesa to make mpesa transcations
#[derive(Debug)]
pub struct Mpesa {
    pub consumerkey: String,
    pub consumersecret: String,
    /*  pub client: Client, */
    pub production_env: Environment,
}
///  Production Environment
#[derive(Debug, Serialize, Display)]
pub enum Environment {
    /// Sandbox app Environment
    #[strum(serialize = "https://sandbox.safaricom.co.ke")]
    Sandbox,
    /// Production app Environment
    #[strum(serialize = "https://api.safaricom.co.ke")]
    Production,
}

#[derive(Error, Debug)]
pub enum MpesaErrors {
    #[error("Invaid authentication")]
    BadCredentials,
}

///Mpesa Client receives th accesstoken response and production_env
#[derive(Deserialize, Debug)]
pub struct MpesaClient {
    pub expires_in: String,
    pub(crate) access_token: String,
    #[serde(skip_deserializing)]
    pub(crate) env: String,
    #[serde(skip_deserializing)]
    pub(crate) client: Client,
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
    pub async fn get_access_token(self) -> Result<MpesaClient, Box<dyn Error>> {
        let client = reqwest::Client::new();
        let resp = client
            .get(format!(
                "{}/oauth/v1/generate?grant_type=client_credentials",
                self.production_env
            ))
            .basic_auth(&self.consumerkey, Some(&self.consumersecret))
            .send()
            .await?;
        // println!("{:#?}", resp);
        //self.access_token = Some(accesstoken.access_token);
        if resp.status().is_success() {
            let mut accesstoken = resp.json::<MpesaClient>().await?;
            accesstoken.env = self.production_env.to_string();
            accesstoken.client = client;
            Ok(accesstoken)
        } else {
            Err(Box::new(MpesaErrors::BadCredentials))
        }
    }
}
