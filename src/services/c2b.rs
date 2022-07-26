use std::error::Error;

use mpesa_sdk::MpesaErrors;
//use serde::Deserialize;

///Customer to business make payment request from Client to Business
///Make payment requests from Client to Business (C2B)
pub struct C2BBuild {
    ///THis is a unique identifier of the transcactiontype:There are two types of
    ///these identifiers:CustomerPayBillOnlin:This is used for Pay Bills shortcode.CustomerBuyGoodsOnline.
    /// This is used for buy goods short code
    commandid: String,
    /// amount to be transcacted.The parameter is required
    amount: i32,
    /// This is the phone number initiating the C2B transaction.
    msisdn: i64,
    /// This is used on CustomerPayBillOnline option only.
    /// This is where a customer is expected to enter a unique bill identifier, e.g. an Account Number.
    billrefnumber: String,
    /// This is the Short Code receiving the amount being transacted.
    shortcode: String,
    /// AccessToken
    token: String,
    //env environment string
    env: String,
}

#[derive(serde::Serialize, Debug)]
pub struct C2B {
    ///THis is a unique identifier of the transcactiontype:There are two types of
    ///these identifiers:CustomerPayBillOnlin:This is used for Pay Bills shortcode.CustomerBuyGoodsOnline.
    /// This is used for buy goods short code
    #[serde(rename = "CommandID")]
    commandid: String,
    /// amount to be transcacted.The parameter is required
    #[serde(rename = "Amount")]
    amount: i32,
    /// This is the phone number initiating the C2B transaction.
    #[serde(rename = "Msisdn")]
    msisdn: i64,
    /// This is used on CustomerPayBillOnline option only.
    /// This is where a customer is expected to enter a unique bill identifier, e.g. an Account Number.
    #[serde(rename = "BillRefNumber")]
    billrefnumber: String,
    /// This is the Short Code receiving the amount being transacted.
    #[serde(rename = "ShortCode")]
    shortcode: String,
}
#[derive(serde::Deserialize, Debug)]
pub struct C2Bresponse {
    #[serde(rename(deserialize = "ResponseCode"))]
    pub responsecode: String,
    #[serde(rename = "OriginatorCoversationID")]
    pub originatorconversationid: String,
    #[serde(rename = "ResponseDescription")]
    pub responsedescription: String,
}
impl C2BBuild {
    // add code here
    pub fn new(
        commandid: String,
        amount: i32,
        phonnumber: i64,
        paybill: String,
        shortcode: String,
        token: String,
        env: String,
    ) -> C2BBuild {
        C2BBuild {
            commandid,
            amount,
            msisdn: phonnumber,
            billrefnumber: paybill,
            shortcode,
            token,
            env,
        }
    }

    pub async fn transact(self) -> Result<C2Bresponse, Box<dyn Error>> {
        let client = reqwest::Client::new();
        let c2b = C2B {
            commandid: self.commandid,
            amount: self.amount,
            msisdn: self.msisdn,
            billrefnumber: self.billrefnumber,
            shortcode: self.shortcode,
        };

        let resp = client
            .post(format!("{}/mpesa/c2b/v1/simulate", self.env))
            .bearer_auth(self.token)
            .json(&c2b)
            .send()
            .await?;
        println!("{:#?}", resp);
        match resp.status().as_str() {
            "200" => return Ok(resp.json::<C2Bresponse>().await?),
            _ => Err(Box::new(MpesaErrors::BadCredentials)),
        }
    }
}
