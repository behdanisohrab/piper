use regex::Regex;
use std::error::Error;

// Function to extract the YouTube video ID from a URL
fn extract_video_id(url: &str) -> Option<String> {
    let re = Regex::new(r"(?i)(?:youtube\.com/watch\?v=|youtu\.be/)([^\s&]+)").unwrap();
    if let Some(captures) = re.captures(url) {
        Some(captures[1].to_string())
    } else {
        None
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Replace with your YouTube URL
    let youtube_url = "https://www.youtube.com/watch?v=YOUR_VIDEO_ID";

    // Extract the video ID from the URL
    if let Some(video_id) = extract_video_id(youtube_url) {
        // Construct the piped.link
        let piped_link = format!("https://piped.video/v/{}", video_id);

        // Print both the YouTube and piped.link
        println!("YouTube Link: {}", youtube_url);
        println!("Piped Link:   {}", piped_link);
    } else {
        eprintln!("Invalid YouTube URL");
    }

    Ok(())
}

