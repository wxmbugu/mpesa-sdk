mod client;
use crate::client::*;
use services::{
    c2b::C2BBuild,
    lipanampesa::{self, LipanaMpesaBuilder},
    register::RegisterUrlsBuilder,
};
mod services;
use std::error::Error;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mpesa = Mpesa::new(
        "2tvPTwLLKkrLzm9L9EFDkf4ezCk7nEY1".to_string(),
        "RBXlqhiDlUYlKqOp".to_string(),
        Environment::Sandbox,
    )
    .get_access_token()
    .await?;
    println!("{:?}", mpesa);
    // .get_access_token()
    // .await?;
    //let access_token = &mpesa.get_access_token().await?;
    //let rt = tokio::runtime::Runtime::new().unwrap();
    // let number: i64 = 254728519199;
    // let shortcode: i32 = 600992;
    // let amount: i32 = 1;
    //let ok = mpesa.get_access_token().await?;
    //println!("{:?}", ok.access_token);
    // let mut c2b = C2BBuild::new(
    //     Some(access_token.access_token),
    //     Some(Environment::Sandbox.to_string()),
    // );
    // let x = &mut c2b;
    // x.msisdn(number)
    //     .commandid("CustomerBuyGoodsOnline".to_string())
    //     /* .billrefnumber("billrefnumber".to_string()) */
    //     .shortcode(shortcode)
    //     .amount(amount)
    //     .transactv2()
    //     .await?;
    // let c2 = c2b.transact().await?;
    //println!("{:?}", c2b.await);
    //let ok = mpesa.get_access_token().await?;

    // let uris = RegisterUrlsBuilder::new(
    //     Some(access_token.access_token),
    //     Some(Environment::Sandbox.to_string()),
    // )
    // .shortcode(600610)
    // .responsetype("Completed".to_string())
    // .confirmationurl("https://2f90-41-90-185-34.eu.ngrok.io".to_string())
    // .valiionurl("https://2f90-41-90-185-34.eu.ngrok.io".to_string())
    // .register()
    // .await?;
    // println!("{:?} whaat", uris);

    let lipanmpesa = LipanaMpesaBuilder::new(mpesa)
        .phone_number(254728519199)
        .transcationtype("sm".to_string())
        .businessshortcode(123)
        .accountreference("me".to_string())
        .amount(10)
        .callbackurl("ok".to_string())
        .transactiondesc("sk".to_string())
        .password("ok".to_string())
        .party_a(254728519199)
        .party_b(2435)
        .stkpush()
        .await;
    println!("{:?}", lipanmpesa);
    Ok(())
}
