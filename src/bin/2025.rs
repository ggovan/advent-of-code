use aoc_2025;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let args: Vec<String> = env::args().collect();
    let day = args.get(1).and_then(|n| n.parse::<usize>().ok());

    aoc_2025::run_all(day).await
}
