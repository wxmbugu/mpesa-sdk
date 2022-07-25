use mpesa_sdk::{AccessToken, Mpesa};

///Customer to business make payment request from Client to Business
///Make payment requests from Client to Business (C2B)
pub struct client2business {
    ///THis is a unique identifier of the transcactiontype:There are two types of
    ///these identifiers:CustomerPayBillOnlin:This is used for Pay Bills shortcode.CustomerBuyGoodsOnline.
    /// This is used for buy goods short code
    commandid: String,
    /// amount to be transcacted.The parameter is required
    amount: i32,
    /// This is the phone number initiating the C2B transaction.
    msisdn: String,
    /// This is used on CustomerPayBillOnline option only.
    /// This is where a customer is expected to enter a unique bill identifier, e.g. an Account Number.
    billrefnumber: String,
    /// This is the Short Code receiving the amount being transacted.
    shortcode: String,
    /// Mpesa App
    mpesa: Option<Mpesa>,
    /// AccessToken
    token: Option<AccessToken>,
}

impl client2business {
    // add code here
    pub fn new(
        commandid: String,
        amount: i32,
        phonnumber: String,
        paybill: String,
        shortcode: String,
    ) -> client2business {
        client2business {
            commandid,
            amount,
            msisdn: phonnumber,
            billrefnumber: paybill,
            shortcode,
            mpesa: None,
            token: None,
        }
    }

    pub fn transact(self) {}
}
