use serde::Serialize;
use std::error::Error;

use crate::client::{MpesaClient, MpesaErrors};

// TDOD:Implement an enum for response type
pub struct RegisterUrlsBuilder {
    /// The shortcode of the organization
    shortcode: Option<i32>,
    /// This parameter specifies what is to happen if for any reason the validation URL is not reachable.
    /// Not that , this is the default value that determines what Mpesa will do in the scenario  that your endpoint is unreachable or is unable to respond on time.
    /// Only two value are allowed: Completed or Cancelled .
    ///Completed meas Mpesa will automatically complete your transaction whereas Cancelled means Mpesa will automatically cancel the transaction,in the event MPesa is unable to reach your ValidationURL
    responsetype: Option<String>,
    /// Thie is the URL that receives the confirmation request from API upon payment completion
    confirmationurl: Option<String>,
    /// This is the URL that receives the validation request from API upon payment submission.
    /// The validation URL is only called if external validation on the registered shortcode is enabled. (By default external validation is disabled)
    validationurl: Option<String>,
    //mpesa client to exectue registering urls
    mpesa: MpesaClient,
}

/// Register validation and confirmation URLs on M-Pesa
#[derive(Serialize)]
struct RegisterUrls {
    /// The shortcode of the organization
    #[serde(rename = "ShortCode")]
    shortcode: i32,
    /// This parameter specifies what is to happen if for any reason the validation URL is not reachable.
    /// Not that , this is the default value that determines what Mpesa will do in the scenario  that your endpoint is unreachable or is unable to respond on time.
    /// Only two value are allowed: Completed or Cancelled .
    ///Completed meas Mpesa will automatically complete your transaction whereas Cancelled means Mpesa will automatically cancel the transaction,in the event MPesa is unable to reach your ValidationURL
    #[serde(rename = "ResponseType")]
    responsetype: String,
    /// Thie is the URL that receives the confirmation request from API upon payment completion
    #[serde(rename = "ConfirmationURL")]
    confirmationurl: String,
    /// This is the URL that receives the validation request from API upon payment submission.
    /// The validation URL is only called if external validation on the registered shortcode is enabled. (By default external validation is disabled)
    #[serde(rename = "ValidationURL")]
    validationurl: String,
}
#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct Registerurlrespone {
    #[serde(rename(deserialize = "ResponseCode"))]
    pub responsecode: String,
    #[serde(rename = "OriginatorCoversationID")]
    pub originatorconversationid: String,
    #[serde(rename = "ResponseDescription")]
    pub responsedescription: String,
}

impl RegisterUrlsBuilder {
    pub fn new(client: MpesaClient) -> RegisterUrlsBuilder {
        RegisterUrlsBuilder {
            shortcode: None,
            responsetype: None,
            confirmationurl: None,
            validationurl: None,
            mpesa: client,
        }
    }
    /// The shortcode of the organization

    pub fn shortcode(&mut self, shortcode: i32) -> &mut Self {
        self.shortcode = Some(shortcode);
        self
    }
    /// This parameter specifies what is to happen if for any reason the validation URL is not reachable.
    /// Not that , this is the default value that determines what Mpesa will do in the scenario  that your endpoint is unreachable or is unable to respond on time.

    pub fn responsetype(&mut self, responsetype: String) -> &mut Self {
        self.responsetype = Some(responsetype);
        self
    }
    /// This is sets the URL that receives the validation request from API upon payment submission.

    pub fn validationurl(&mut self, validationurl: String) -> &mut Self {
        self.validationurl = Some(validationurl);
        self
    }
    /// This is sets URL that receives the confirmation request from API upon payment completion

    pub fn confirmationurl(&mut self, confirmationurl: String) -> &mut Self {
        self.confirmationurl = Some(confirmationurl);
        self
    }
    pub async fn register(&self) -> Result<Registerurlrespone, Box<dyn Error>> {
        let registerurl = RegisterUrls {
            shortcode: self.shortcode.ok_or("Short code required")?,
            responsetype: self
                .responsetype
                .as_ref()
                .ok_or("response type required")?
                .to_string(),
            confirmationurl: self
                .confirmationurl
                .as_ref()
                .ok_or("confirmation url required")?
                .to_string(),
            validationurl: self
                .validationurl
                .as_ref()
                .ok_or("validation url required")?
                .to_string(),
        };

        let  resp = self
            .mpesa
            .client
            .post(format!("{}/mpesa/c2b/v1/registerurl", self.mpesa.env))
            .bearer_auth(self.mpesa.access_token.to_string())
            .json(&registerurl)
            .send()
            .await?;
        match resp.status().as_str() {
            "200" => return Ok(resp.json::<Registerurlrespone>().await?),
            _ => Err(Box::new(MpesaErrors::BadCredentials)),
        }
    }
}
