mod ui;
mod utils;

use std::error::Error;
// Function to extract the YouTube video ID from a URL
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    ui::run_gui().await;

    Ok(())
}
