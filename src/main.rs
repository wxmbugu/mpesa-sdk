use mpesa_sdk::{Environment, Mpesa};
use services::c2b::C2BBuild;
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
    let ok = mpesa.get_access_token().await?;
    println!("{:?}", ok.access_token);
    let c2b = C2BBuild::new(
        "CustomerBuyGoodsOnline".to_string(),
        1,
        254728519199,
        "".to_string(),
        "600247".to_string(),
        ok.access_token,
        mpesa.production_env.to_string(),
    );
    let c2 = c2b.transact().await?;
    println!("{:?}",c2);
    Ok(())
}
