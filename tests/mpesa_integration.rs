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
        .shortcode(600992)
        .responsetype("okkkk".to_string())
        .register()
        .await
        .expect("Error");

    assert_eq!("Success", c2bregister.responsedescription);
}

#[tokio::test]
#[ignore = "test keeps on failing find some test data maybe"]
async fn lipanampesa() {
    let mpesa = mpesa().await;
    let c2bregister = LipanaMpesaBuilder::new(mpesa)
        .phone_number("254728519199".to_string())
        .transcationtype("sm".to_string())
        .businessshortcode(123)
        .party_a("254718778852".to_string())
        .party_b(2435)
        .password("12323434".to_string())
        .amount(2)
        .stkpush()
        .await
        .expect("Error");

    assert_eq!("Success", c2bregister.responsedescription);
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
