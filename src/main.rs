use std::error::Error;

use mpesa_sdk::{Environment, Mpesa};
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mpesa = Mpesa::new(
        "2tvPTwLLKkrLzm9L9EFDkf4ezCk7nEY1".to_string(),
        " RBXlqhiDlUYlKqOp".to_string(),
        Environment::Sandbox,
    );

    //let rt = tokio::runtime::Runtime::new().unwrap();
    mpesa.get_access_token().await?;
    Ok(())
}
