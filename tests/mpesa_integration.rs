use mpesa_sdk::{
    self, c2b::C2BBuild, lipanampesa::LipanaMpesaBuilder, register::RegisterUrlsBuilder, CommandID,
    Mpesa, MpesaClient,
};

// macro_rules! aw {
//     ($e:expr) => {
//         tokio_test::block_on($e)
//     };
// }

async fn mpesa() -> MpesaClient {
    Mpesa::new(
        "2tvPTwLLKkrLzm9L9EFDkf4ezCk7nEY1".to_string(),
        "RBXlqhiDlUYlKqOp".to_string(),
        mpesa_sdk::Environment::Sandbox,
    )
    .get_access_token()
    .await
    .unwrap()
}

#[tokio::test]
async fn register() {
    let mpesa = mpesa().await;
    let c2bregister = RegisterUrlsBuilder::new(mpesa)
        .validationurl("https://someurl.com".to_string())
        .confirmationurl("https://anotherurl.com".to_string())
        .shortcode(600610)
        .responsetype("Cancelled".to_string())
        .register()
        .await
        .expect("Error");

    assert_eq!("Success", c2bregister.responsedescription);
}

#[tokio::test]
//#[ignore = "test keeps on failing, find some test data."]
async fn lipanampesa() {
    let mpesa = Mpesa::new(
        "nmOLCEGwFYRBDZ9kOXmZOheQnU0jvBGB".to_string(),
        "YyNPLB5Si7JzrOVm".to_string(),
        mpesa_sdk::Environment::Sandbox,
    )
    .get_access_token()
    .await
    .unwrap();
    let lipanampesa = LipanaMpesaBuilder::new(mpesa)
        .phone_number(254708374149)
        .transcationtype(CommandID::CustomerPayBillOnline.to_string())
        .businessshortcode(174379)
        .party_a(254708374149)
        .party_b(174379)
        .password("bfb279f9aa9bdbcf158e97dd71a467cd2e0c893059b10f78e6b72ada1ed2c919".to_string())
        .amount(1)
        .callbackurl("https://mydomain.com/pat".to_string())
        .accountreference("Test".to_string())
        .transactiondesc("Test".to_string())
        .stkpush()
        .await
        .expect("Error");
    assert_eq!(
        "Success. Request accepted for processing",
        lipanampesa.responsedescription
    );
}

#[tokio::test]
async fn c2b() {
    let mpesa = mpesa().await;
    let c2b = C2BBuild::new(mpesa)
        .commandid(CommandID::CustomerBuyGoodsOnline)
        .shortcode(600992)
        .amount(1)
        .msisdn(254728519199)
        .transact()
        .await
        .expect("Error");
    println!("{:?}", c2b);
    assert_eq!(
        "Accept the service request successfully.",
        c2b.responsedescription
    );
}

#[tokio::test]
async fn c2bv2() {
    let mpesa = mpesa().await;
    let c2b = C2BBuild::new(mpesa)
        .commandid(CommandID::CustomerBuyGoodsOnline)
        .shortcode(600992)
        .amount(1)
        .msisdn(254728519199)
        .transactv2()
        .await
        .expect("Error");
    println!("{:?}", c2b);
    assert_eq!(
        "Accept the service request successfully.",
        c2b.responsedescription
    );
}
