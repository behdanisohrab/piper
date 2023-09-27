mod ui;
mod utils;

use std::error::Error;
use std::io;
// Function to extract the YouTube video ID from a URL
use utils::export_piper_link;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    ui::run_gui().await;
    let youtube_url = String::new();


    Ok(())
}
