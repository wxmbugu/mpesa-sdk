use std::error::Error;

use crate::client::{MpesaClient, MpesaErrors::BadCredentials};

///Customer to business make payment request from Client to Business
///Make payment requests from Client to Business (C2B)
pub struct C2BBuild {
    ///THis is a unique identifier of the transcactiontype:There are two types of
    ///these identifiers:CustomerPayBillOnlin:This is used for Pay Bills shortcode.CustomerBuyGoodsOnline.
    /// This is used for buy goods short code
    commandid: Option<String>,
    /// amount to be transcacted.The parameter is required
    amount: Option<i32>,
    /// This is the phone number initiating the C2B transaction.
    msisdn: Option<i64>,
    /// This is used on CustomerPayBillOnline option only.
    /// This is where a customer is expected to enter a unique bill identifier, e.g. an Account Number.
    billrefnumber: String,
    /// This is the Short Code receiving the amount being transacted.
    shortcode: Option<i32>,
    mpesa: MpesaClient,
}

#[derive(serde::Serialize, Debug)]
struct C2B {
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
    shortcode: i32,
}
#[derive(serde::Deserialize, Debug)]
pub struct C2Bresponse {
    #[serde(rename = "ConversationID", skip_serializing_if = "ConversationID")]
    pub conversationid: String,
    #[serde(rename = "OriginatorCoversationID")]
    pub originatorconversationid: String,
    #[serde(rename = "ResponseDescription")]
    pub responsedescription: String,
}
impl C2BBuild {
    // add code here

    //     commandid: String,
    //     amount: i32,
    //     phonnumber: i64,
    //     paybill: String
    pub fn new(client: MpesaClient) -> C2BBuild {
        C2BBuild {
            commandid: None,
            amount: None,
            msisdn: None,
            billrefnumber: "".to_string(),
            shortcode: None,
            mpesa: client,
        }
    }
    //TODO: Set an enum type for CommandID
    ///  Sets CommandID either : CustomerPayBillOnline or CustomerBuyGoodsOnline
    /// This is a unique identifier of the transaction type: There are two types of these Identifiers:CustomerPayBillOnline: This is used for Pay Bills
    ///shortcodes.CustomerBuyGoodsOnline: This is used for Buy Goods shortcodes.
    pub fn commandid(&mut self, commandid: String) -> &mut Self {
        self.commandid = Some(commandid);
        self
    }
    /// BillRefNumber: This is used on CustomerPayBillOnline option only.
    /// This is where a customer is expected to enter a unique bill identifier, e.g an Account Number.
    pub fn billrefnumber(&mut self, billrefnumber: String) -> &mut Self {
        self.billrefnumber = billrefnumber;
        self
    }
    /// Sets the phone number initiating the C2B transaction.
    pub fn msisdn(&mut self, msisdn: i64) -> &mut Self {
        self.msisdn = Some(msisdn);
        self
    }
    /// Sets the amount being transacted.
    pub fn amount(&mut self, amount: i32) -> &mut Self {
        self.amount = Some(amount);
        self
    }
    /// Sets the  Short Code receiving the amount being transacted.
    pub fn shortcode(&mut self, shortcode: i32) -> &mut Self {
        self.shortcode = Some(shortcode);
        self
    }
    /// V1:C2B-> Make payment requests from Client to Business (C2B). Simulate is available on sandbox only
    pub async fn transact(&self) -> Result<C2Bresponse, Box<dyn Error>> {
        let c2b = C2B {
            commandid: self
                .commandid
                .as_ref()
                .ok_or("CommandID Required")?
                .to_string(),
            amount: self.amount.ok_or("Ammount Required")?,
            msisdn: self.msisdn.ok_or("Contact Required")?,
            billrefnumber: self.billrefnumber.to_string(),
            shortcode: self.shortcode.ok_or("Shortcode Required")?,
        };
        let resp = self
            .mpesa
            .client
            .post(format!("{}/mpesa/c2b/v1/simulate", self.mpesa.env))
            .bearer_auth(self.mpesa.access_token.to_string())
            .json(&c2b)
            .send()
            .await?;
        //println!("{:#?}", resp);
        match resp.status().as_str() {
            "200" => return Ok(resp.json::<C2Bresponse>().await?),
            _ => Err(Box::new(BadCredentials)),
        }
    }
    /// V2:C2B-> Make payment requests from Client to Business (C2B). Simulate is available on sandbox only
    pub async fn transactv2(&self) -> Result<C2Bresponse, Box<dyn Error>> {
        let c2b = C2B {
            commandid: self
                .commandid
                .as_ref()
                .ok_or("CommandID Required")?
                .to_string(),
            amount: self.amount.ok_or("Ammount Required")?,
            msisdn: self.msisdn.ok_or("Contact Required")?,
            billrefnumber: self.billrefnumber.to_string(),
            shortcode: self.shortcode.ok_or("Shortcode Required")?,
        };

        let resp = self
            .mpesa
            .client
            .post(format!("{}/mpesa/c2b/v2/simulate", self.mpesa.env))
            .bearer_auth(self.mpesa.access_token.to_string())
            .json(&c2b)
            .send()
            .await?;
        //println!("{:#?}", resp);
        match resp.status().as_str() {
            "200" => return Ok(resp.json::<C2Bresponse>().await?),
            _ => Err(Box::new(BadCredentials)),
        }
    }
}
