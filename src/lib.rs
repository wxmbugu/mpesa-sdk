use hyper::{body::HttpBody, Client};
use hyper_tls::HttpsConnector;
use std::io::{stdout, Write};

///! Mpesa to make mpesa transcations
#[derive(Debug)]
pub struct Mpesa {
    consumerkey: String,
    consumersecret: String,
    production_env: Environment,
}
///!  Production Environment
#[derive(Debug)]
pub enum Environment {
    ///!Sandbox app Environment
    Sandbox,
    ///! Live app Environment
    Production,
}

//const SANDBOXBASEURL: &str = "https://sandbox.safaricom.co.ke";

//const PRODUCTIONBASEURL: &str = "https://api.safaricom.co.ke";

impl Mpesa {
    ///! Creates an Mpesa app either sandbox or live app
    pub fn new(key: String, secret: String, env: Environment) -> Mpesa {
        // match env {
        //   Environment::Sandbox => SANDBOXBASEURL,
        // Environment::Production => PRODUCTIONBASEURL,
        // };
        Mpesa {
            consumerkey: key,
            consumersecret: secret,
            production_env: env,
        }
    }
    ///! Returns a token to be used to aythenticate an app
    ///!  
    pub async fn get_access_token(&self) -> Result<(), Box<dyn std::error::Error>> {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);

        let uri = "https://sandbox.safaricom.co.ke/oauth/v1/generate?grant_type=client_credentials"
            .parse()?;

        let mut resp = client.get(uri).await?;
        println!("Response: {}", resp.status());

        while let Some(chunk) = resp.body_mut().data().await {
            stdout().write_all(&chunk?)?;
        }

        Ok(())
        //resp.
    }
}
