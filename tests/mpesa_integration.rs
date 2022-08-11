use mpesa_sdk::{self, c2b::C2BBuild, CommandID, Mpesa};

// macro_rules! aw {
//     ($e:expr) => {
//         tokio_test::block_on($e)
//     };
// }

#[tokio::test]
async fn mpesa() {
    let mpesa = Mpesa::new(
        "2tvPTwLLKkrLzm9L9EFDkf4ezCk7nEY1".to_string(),
        "RBXlqhiDlUYlKqOp".to_string(),
        mpesa_sdk::Environment::Sandbox,
    )
    .get_access_token()
    .await
    .expect("Error");
    assert_eq!("3599", mpesa.expires_in);
}

#[tokio::test]
async fn c2b() {
    let mpesa = Mpesa::new(
        "2tvPTwLLKkrLzm9L9EFDkf4ezCk7nEY1".to_string(),
        "RBXlqhiDlUYlKqOp".to_string(),
        mpesa_sdk::Environment::Sandbox,
    )
    .get_access_token()
    .await
    .expect("Error");
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
