use mpesa_sdk::{Environment, Mpesa};
use services::{c2b::C2BBuild, register::RegisterUrlsBuilder};
mod services;
use std::error::Error;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mpesa = Mpesa::new(
        "2tvPTwLLKkrLzm9L9EFDkf4ezCk7nEY1".to_string(),
        "RBXlqhiDlUYlKqOp".to_string(),
        Environment::Sandbox,
    );
    //let rt = tokio::runtime::Runtime::new().unwrap();
    let number: i64 = 254728519199;
    let shortcode: i32 = 600992;
    let amount: i32 = 1;
    //    let ok = mpesa.get_access_token().await?.;
    //println!("{:?}", ok.access_token);
    let mut c2b = C2BBuild::new(
        Some(mpesa.get_access_token().await?.access_token),
        Some(mpesa.production_env.to_string()),
    );
    let x = &mut c2b;
    x.msisdn(number)
        .commandid("CustomerBuyGoodsOnline".to_string())
        /* .billrefnumber("billrefnumber".to_string()) */
        .shortcode(shortcode)
        .amount(amount)
        .transact()
        .await?;
    // let c2 = c2b.transact().await?;
    //println!("{:?}", c2b.await);
    let ok = mpesa.get_access_token().await?;
    let uris = RegisterUrlsBuilder::new(
        Some(ok.access_token),
        Some(mpesa.production_env.to_string()),
    )
    .shortcode(600610)
    .responsetype("Completed".to_string())
    .confirmationurl("https://2f90-41-90-185-34.eu.ngrok.io".to_string())
    .validationurl("https://2f90-41-90-185-34.eu.ngrok.io".to_string())
    .register()
    .await;
    println!("{:?}", uris);
    Ok(())
}
