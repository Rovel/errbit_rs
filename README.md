# errbit_rs

A Rust client for Errbit error catcher.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
errbit_rs = { path = "/home/rovel/dev/errbit_rs" }
```

Example:

```rust
use errbit_rs::ErrbitClient;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = ErrbitClient::new("https://your-errbit-instance.com/api/v3/projects/YOUR_PROJECT_ID/notices", "YOUR_API_KEY");

    // Simulate an error
    let simulated_error = std::io::Error::new(std::io::ErrorKind::Other, "Simulated error");

    client.notify(&simulated_error).await?;

    Ok(())
}
```
