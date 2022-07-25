use chrono::prelude::*;

///LIPA NA M-PESA ONLINE API also know as M-PESA express (STK Push) is a Merchant/Business initiated C2B (Customer to Business) Payment.
pub struct LipanaMpesa {
    ///This is organizations shortcode (Paybill or Buygoods
    ///- A 5 to 6 digit account number) used to identify an organization and receive the transaction.
    pub business_short_code: i32,
    ///This is the password used for encrypting the request sent: A base64 encoded string.
    ///(The base64 string is a combination of Shortcode+Passkey+Timestamp)
    pub password: String,
    ///This is the Timestamp of the transaction, normally in the format of YEAR+MONTH+DATE+HOUR+MINUTE+SECOND (YYYYMMDDHHMMSS)
    ///Each part should be at least two digits apart from the year which takes four digits.
    pub timestamp: DateTime<Utc>,
    ///This is the transaction type that is used to identify the transaction when sending the request to M-PESA.
    ///The transaction type for M-PESA Express is "CustomerPayBillOnline" for PayBill Numbers and "CustomerBuyGoodsOnline"
    ///for Till Numbers.
    pub transcactiontype: String,
    ///This is the Amount transacted normally a numeric value. Money that customer pays to the Shorcode.
    ///Only whole numbers are supported.
    pub amount: i64,
    ///The phone number sending money.
    ///The parameter expected is a Valid Safaricom Mobile Number that is M-PESA registered in the format 2547XXXXXXXX
    pub party_a: i32,
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
