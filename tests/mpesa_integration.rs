use mpesa_sdk::{self, Mpesa};

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
