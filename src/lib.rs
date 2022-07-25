//use hyper::header;
//use hyper::Client;
//use hyper::Request;
//use hyper_tls::HttpsConnector;
//use std::io::{stdout, Write};
extern crate serde;
//use reqwest::StatusCode;
//use reqwest::StatusCode;
use serde::Deserialize;
//use tokio::sync::mpsc::error;
use std::string::ToString;
use strum_macros::Display;
use thiserror::Error;
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
pub struct AccessTokenResponse {
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
    pub async fn get_access_token(
        &self,
    ) -> Result<AccessTokenResponse, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let resp = client
            .get(format!(
                "{}/oauth/v1/generate?grant_type=client_credentials",
                self.production_env.to_string()
            ))
            .basic_auth(&self.consumerkey, Some(&self.consumersecret))
            .send()
            .await?;
        match resp.status().as_str() {
            "200" => return Ok(resp.json::<AccessTokenResponse>().await?),
            _ => return Err(Box::new(MpesaErrors::BadCredentials)),
        }
    }
}
