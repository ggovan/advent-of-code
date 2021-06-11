use football_puzzle;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    football_puzzle::run_all()
}
