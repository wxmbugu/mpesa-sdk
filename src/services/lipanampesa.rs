extern crate base64;

use base64::encode;
use reqwest::Client;
use std::error::Error;

use chrono::prelude::*;

use serde::Serialize;

use crate::client::MpesaClient;
///LIPA NA M-PESA ONLINE API also know as M-PESA express (STK Push) is a Merchant/Business initiated C2B (Customer to Business) Payment.
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
    party_a: Option<i64>,
    ///The organization receiving the funds. The parameter expected is a 5 to 6 digit as defined on the Shortcode description above.
    ///This can be the same as BusinessShortCode value above.
    party_b: Option<i32>,
    ///The Mobile Number to receive the STK Pin Prompt. This number can be the same as PartyA valueT: above.
    phone_number: Option<i64>,
    ///A CallBack URL is a valid secure URL that is used to receive notifications from M-Pesa API.
    ///It is the endpoint to which the results will be sent by M-Pesa API.
    callbackurl: Option<String>,
    ///Account Reference: This is an Alpha-Numeric parameter that is defined by your system as an Identifier of the transaction for CustomerPayBillOnline transaction type.
    //Along with the business name, this value is also displayed to the customer in the STK Pin Prompt message. Maximum of 12 characters.
    accountreference: Option<String>,
    ///This is any additional information/comment that can be sent along with the request from your system. Maximum of 13 Characters.
    transactiondesc: Option<String>,
    production_env: String,
    accesstoken: String,
    client: Client,
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
    ///The phone n:+umber sending money.
    ///The parameter expected is a Valid Safaricom Mobile Number that is M-PESA registered in the format 2547XXXXXXXX
    pub party_a: i64,
    ///The organization receiving the funds. The parameter expected is a 5 to 6 digit as defined on the Shortcode description above.
    ///This can be the same as BusinessShortCode value above.
    pub party_b: i32,
    ///The Mobile Number to receive the STK Pin Prompt. This number can be the same as PartyA valueT: above.
    pub phone_number: i64,
    ///A CallBack URL is a valid secure URL that is used to receive notifications from M-Pesa API.
    ///It is the endpoint to which the results will be sent by M-Pesa API.
    pub callbackurl: String,
    ///Account Reference: This is an Alpha-Numeric parameter that is defined by your system as an Identifier of the transaction for CustomerPayBillOnline transaction type.
    //Along with the business name, this value is also displayed to the customer in the STK Pin Prompt message. Maximum of 12 characters.
    pub accountreference: String,
    ///This is any additional information/comment that can be sent along with the request from your system. Maximum of 13 Characters.
    pub transactiondesc: String,
}
impl LipanaMpesaBuilder {
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
            production_env: client.env,
            accesstoken: client.access_token,
            client: client.client,
        }
    }
    pub fn businessshortcode(&mut self, business_short_code: i32) -> &mut Self {
        self.business_short_code = Some(business_short_code);
        self
    }
    pub fn password(&mut self, password: String) -> &mut Self {
        self.password = Some(encode(
            self.business_short_code.unwrap().to_string()
                + &password
                + &self.timestamp.format("%Y%M%d%H%M%S").to_string(),
        ));
        self
    }
    pub fn transcationtype(&mut self, transcationtype: String) -> &mut Self {
        self.transcactiontype = Some(transcationtype);
        self
    }
    pub fn amount(&mut self, amount: i64) -> &mut Self {
        self.amount = Some(amount);
        self
    }
    pub fn party_a(&mut self, phone_number: i64) -> &mut Self {
        self.party_a = Some(phone_number);
        self
    }
    pub fn party_b(&mut self, party_b: i32) -> &mut Self {
        self.party_b = Some(party_b);
        self
    }
    pub fn phone_number(&mut self, phone_number: i64) -> &mut Self {
        self.phone_number = Some(phone_number);
        self
    }
    pub fn callbackurl(&mut self, url: String) -> &mut Self {
        self.callbackurl = Some(url);
        self
    }
    pub fn accountreference(&mut self, account_reference: String) -> &mut Self {
        self.accountreference = Some(account_reference);
        self
    }
    pub fn transactiondesc(&mut self, description: String) -> &mut Self {
        self.transactiondesc = Some(description);
        self
    }

    pub async fn stkpush(&mut self) -> Result<(), Box<dyn Error>> {
        let lipanampesa = LipanaMpesa {
            business_short_code: self.business_short_code.unwrap(),
            password: self.password.as_ref().ok_or("password error")?.to_string(),
            timestamp: self.timestamp.format("%Y%M%d%H%M%S").to_string(),
            transcactiontype: self
                .transcactiontype
                .as_ref()
                .ok_or("transcactiontype required")?
                .to_string(),
            amount: self.amount.unwrap(),
            party_a: self.party_a.unwrap(),
            party_b: self.party_b.unwrap(),
            phone_number: self.phone_number.unwrap(),
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
            transactiondesc: self.transactiondesc.as_ref().unwrap().to_string(),
        };
        let mut resp = self
            .client
            .post(format!(
                "{}/mpesa/stkpush/v1/processrequest",
                self.production_env
            ))
            .bearer_auth(self.accesstoken.to_string())
            .json(&lipanampesa)
            .send()
            .await?;
        while let Some(chunks) = resp.chunk().await? {
            println!("{:?}", &chunks)
        }
        Ok(())
    }
}
