use mpesa_sdk::{Environment, Mpesa};
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
    println!("{:?}", ok);

    Ok(())
}
