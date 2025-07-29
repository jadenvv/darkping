mod cli; 
use tokio;
#[tokio::main]
async fn main() {
    cli::start_cli().await;
}
