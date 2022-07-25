///Customer to business make payment request from Client to Business
///Make payment requests from Client to Business (C2B)
pub struct Client2Business {
    ///THis is a unique identifier of the transcactiontype:There are two types of
    ///these identifiers:CustomerPayBillOnlin:This is used for Pay Bills shortcode.CustomerBuyGoodsOnline.
    //This is used for buy goods short code
    pub commandid: String,
    ///amount to be transcacted.The parameter is required
    pub amount: i64,
    ///This is the phone number initiating the C2B transaction.
    pub msisdn: String,
    /// This is used on CustomerPayBillOnline option only.
    //This is where a customer is expected to enter a unique bill identifier, e.g. an Account Number.
    pub billrefnumber: String,
    ///This is the Short Code receiving the amount being transacted.
    pub shortcode: String,
}
