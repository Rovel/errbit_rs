use errbit_rs::ErrbitClient;
use std::error::Error;

#[tokio::test]
async fn test_errbit_client() -> Result<(), Box<dyn Error>> {
    let url = std::env::var("ERRBIT_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());
        let api_key = std::env::var("ERRBIT_API_KEY").unwrap_or_else(|_| "generate-app-key-in-errbit".to_string());
    let client = ErrbitClient::new(
        url.as_str(),
        api_key.as_str(),
    );

    let simulated_error = std::io::Error::new(std::io::ErrorKind::Other, "Simulated error");
    client.notify(&simulated_error).await?;

    Ok(())
}
