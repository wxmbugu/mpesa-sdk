extern crate base64;

use std::error::Error;

use chrono::prelude::*;

use serde::Serialize;

use crate::{client::MpesaClient, MpesaErrors};
///LIPA NA M-PESA ONLINE API also know as M-PESA express (STK Push) is a Merchant/Business initiated C2B (Customer to Business) Payment.
#[derive(Debug)]
pub struct LipanaMpesaBuilder {
    ///This is organizations shortcode (Paybill or Buygoods
    ///- A 5 to 6 digit account number) used to identify an organization and receive the transaction.
    business_short_code: Option<i32>,
    ///This is the password used for encrypting the request sent: A base64 encoded string.
    ///(The base64 string is a combination of Shortcode+Passkey+Timestamp)
    password: Option<String>,
    ///This is the Timestamp of the transaction, normally in the format of YEAR+MONTH+DATE+HOUR+MINUTE+SECOND (YYYYMMDDHHMMSS)
    ///Each part should be at least two digits apart from the year which takes four digits.
    timestamp: DateTime<Utc>,
    ///This is the transaction type that is used to identify the transaction when sending the request to M-PESA.
    ///The transaction type for M-PESA Express is "CustomerPayBillOnline" for PayBill Numbers and "CustomerBuyGoodsOnline"
    ///for Till Numbers.
    transcactiontype: Option<String>,
    ///This is the Amount transacted normally a numeric value. Money that customer pays to the Shorcode.
    ///Only whole numbers are supported.
    amount: Option<i64>,
    ///The phone number sending money.
    ///The parameter expected is a Valid Safaricom Mobile Number that is M-PESA registered in the format 2547XXXXXXXX
    party_a: Option<String>,
    ///The organization receiving the funds. The parameter expected is a 5 to 6 digit as defined on the Shortcode description above.
    ///This can be the same as BusinessShortCode value above.
    party_b: Option<i32>,
    ///The Mobile Number to receive the STK Pin Prompt. This number can be the same as PartyA valueT: above.
    phone_number: Option<String>,
    ///A CallBack URL is a valid secure URL that is used to receive notifications from M-Pesa API.
    ///It is the endpoint to which the results will be sent by M-Pesa API.
    callbackurl: Option<String>,
    ///Account Reference: This is an Alpha-Numeric parameter that is defined by your system as an Identifier of the transaction for CustomerPayBillOnline transaction type.
    //Along with the business name, this value is also displayed to the customer in the STK Pin Prompt message. Maximum of 12 characters.
    accountreference: Option<String>,
    ///This is any additional information/comment that can be sent along with the request from your system. Maximum of 13 Characters.
    transactiondesc: Option<String>,
    ///Receives MpesaClient to execute stk push actions
    mpesa: MpesaClient,
}
#[derive(Serialize)]
///LIPA NA M-PESA ONLINE API also know as M-PESA express (STK Push) is a Merchant/Business initiated C2B (Customer to Business) Payment.
struct LipanaMpesa {
    ///This is organizations shortcode (Paybill or Buygoods
    ///- A 5 to 6 digit account number) used to identify an organization and receive the transaction.
    pub business_short_code: i32,
    ///This is the password used for encrypting the request sent: A base64 encoded string.
    ///(The base64 string is a combination of Shortcode+Passkey+Timestamp)
    pub password: String,
    ///This is the Timestamp of the transaction, normally in the format of YEAR+MONTH+DATE+HOUR+MINUTE+SECOND (YYYYMMDDHHMMSS)
    ///Each part should be at least two digits apart from the year which takes four digits.
    pub timestamp: String,
    ///This is the transaction type that is used to identify the transaction when sending the request to M-PESA.
    ///The transaction type for M-PESA Express is "CustomerPayBillOnline" for PayBill Numbers and "CustomerBuyGoodsOnline"
    ///for Till Numbers.
    pub transcactiontype: String,
    ///This is the Amount transacted normally a numeric value. Money that customer pays to the Shorcode.
    ///Only whole numbers are supported.
    pub amount: i64,
    ///The phone number sending money.
    ///The parameter expected is a Valid Safaricom Mobile Number that is M-PESA registered in the format 2547XXXXXXXX
    pub party_a: String,
    ///The organization receiving the funds. The parameter expected is a 5 to 6 digit as defined on the Shortcode description above.
    ///This can be the same as BusinessShortCode value above.
    pub party_b: i32,
    ///The Mobile Number to receive the STK Pin Prompt. This number can be the same as PartyA valueT: above.
    pub phone_number: String,
    ///A CallBack URL is a valid secure URL that is used to receive notifications from M-Pesa API.
    ///It is the endpoint to which the results will be sent by M-Pesa API.
    pub callbackurl: String,
    ///Account Reference: This is an Alpha-Numeric parameter that is defined by your system as an Identifier of the transaction for CustomerPayBillOnline transaction type.
    //Along with the business name, this value is also displayed to the customer in the STK Pin Prompt message. Maximum of 12 characters.
    pub accountreference: String,
    ///This is any additional information/comment that can be sent along with the request from your system. Maximum of 13 Characters.
    pub transactiondesc: String,
}
#[derive(serde::Deserialize, Debug)]
pub struct LipanaMpesaResponse {
    #[serde(rename = "MerchantRequestID")]
    pub merchantrequestid: String,
    #[serde(rename = "CheckoutRequestID")]
    pub checkoutrequesid: String,
    #[serde(rename(deserialize = "ResponseCode"))]
    pub responsecode: String,
    #[serde(rename = "ResponseDescription")]
    pub responsedescription: String,
    #[serde(rename = "CustomerMessage")]
    pub customermessage: String,
}

impl LipanaMpesaBuilder {
    ///initiates the LipanaMpesaBuilder
    pub fn new(client: MpesaClient) -> LipanaMpesaBuilder {
        LipanaMpesaBuilder {
            business_short_code: None,
            password: None,
            timestamp: Utc::now(),
            transcactiontype: None,
            amount: None,
            party_a: None,
            party_b: None,
            phone_number: None,
            callbackurl: None,
            accountreference: None,
            transactiondesc: None,
            mpesa: client,
        }
    }
    ///This is organizations shortcode (Paybill or Buygoods
    ///- A 5 to 6 digit account number) used to identify an organization and receive the transaction.
    pub fn businessshortcode(&mut self, business_short_code: i32) -> &mut Self {
        self.business_short_code = Some(business_short_code);
        self
    }
    ///This is the password used for encrypting the request sent:
    pub fn password(&mut self, password: String) -> &mut Self {
        self.password = Some(base64::encode(
            self.business_short_code.unwrap().to_string()
                + &password
                + &self.timestamp.format("%Y%M%d%H%M%S").to_string(),
        ));
        self
    }
    ///The transaction type for M-PESA Express is "CustomerPayBillOnline" for PayBill Numbers and "CustomerBuyGoodsOnline"   
    pub fn transcationtype(&mut self, transcationtype: String) -> &mut Self {
        self.transcactiontype = Some(transcationtype);
        self
    }
    ///This is the Amount transacted normally a numeric value. Money that customer pays to the Shorcode.
    pub fn amount(&mut self, amount: i64) -> &mut Self {
        self.amount = Some(amount);
        self
    }
    ///The phone number sending money.
    ///The parameter expected is a Valid Safaricom Mobile Number that is M-PESA registered in the format 2547XXXXXXXX

    pub fn party_a(&mut self, phone_number: String) -> &mut Self {
        self.party_a = Some(phone_number);
        self
    }
    ///The organization receiving the funds. The parameter expected is a 5 to 6 digit
    pub fn party_b(&mut self, party_b: i32) -> &mut Self {
        self.party_b = Some(party_b);
        self
    }
    ///The Mobile Number to receive the STK Pin Prompt. This number can be the same as PartyA valueT: above.

    pub fn phone_number(&mut self, phone_number: String) -> &mut Self {
        self.phone_number = Some(phone_number);
        self
    }
    ///A CallBack URL is a valid secure URL that is used to receive notifications from M-Pesa API.
    ///It is the endpoint to which the results will be sent by M-Pesa API.

    pub fn callbackurl(&mut self, url: String) -> &mut Self {
        self.callbackurl = Some(url);
        self
    }
    ///Account Reference: This is an Alpha-Numeric parameter that is defined by your system as an Identifier of the transaction for CustomerPayBillOnline transaction type.
    //Along with the business name, this value is also displayed to the customer in the STK Pin Prompt message. Maximum of 12 characters.

    pub fn accountreference(&mut self, account_reference: String) -> &mut Self {
        self.accountreference = Some(account_reference);
        self
    }
    ///This is any additional information/comment that can be sent along with the request from your system. Maximum of 13 Characters.

    pub fn transactiondesc(&mut self, description: String) -> &mut Self {
        self.transactiondesc = Some(description);
        self
    }
    ///sets the API required parameters and sends the API REQUEST to Mpesa
    pub async fn stkpush(&mut self) -> Result<LipanaMpesaResponse, Box<dyn Error>> {
        let lipanampesa = LipanaMpesa {
            business_short_code: self.business_short_code.unwrap(),
            password: self
                .password
                .as_ref()
                .ok_or("password required")?
                .to_string(),
            timestamp: self.timestamp.format("%Y%M%d%H%M%S").to_string(),
            transcactiontype: self
                .transcactiontype
                .as_ref()
                .ok_or("transcactiontype required")?
                .to_string(),
            amount: self.amount.ok_or("amount required")?,
            party_a: self.party_a.as_ref().ok_or("party_a required")?.to_string(),
            party_b: self.party_b.ok_or("party_b required")?,
            phone_number: self
                .phone_number
                .as_ref()
                .ok_or("phone_number required")?
                .to_string(),
            callbackurl: self
                .callbackurl
                .as_ref()
                .ok_or("callbackurl required")?
                .to_string(),
            accountreference: self
                .accountreference
                .as_ref()
                .ok_or("accountreference Reference required")?
                .to_string(),
            transactiondesc: self
                .transactiondesc
                .as_ref()
                .ok_or("transaction description required")?
                .to_string(),
        };
        let resp = self
            .mpesa
            .client
            .post(format!(
                "{}/mpesa/stkpush/v1/processrequest",
                self.mpesa.env
            ))
            .bearer_auth(self.mpesa.access_token.to_string())
            .json(&lipanampesa)
            .send()
            .await?;
        match resp.status().as_str() {
            "200" => return Ok(resp.json::<LipanaMpesaResponse>().await?),
            _ => Err(Box::new(MpesaErrors::BadCredentials)),
        }
    }
}
