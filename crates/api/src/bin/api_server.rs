use voip_api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting API server on 127.0.0.1:3000...");
    voip_api::serve("127.0.0.1:3000").await?;
    Ok(())
}