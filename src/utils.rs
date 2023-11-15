use regex::Regex;

pub fn extract_video_id(url: &str) -> Option<String> {
    let re = Regex::new(r"(?i)(?:youtube\.com/watch\?v=|youtu\.be/)([^\s&]+)").unwrap();
    re.captures(url).map(|captures| captures[1].to_string())
}
pub fn export_piper_link(youtube_url:&str)->Option<String>{
    if let Some(video_id) = extract_video_id(youtube_url) {
        // Construct the piped.link
        let piper_link = format!("https://piped.video/v/{}", video_id);
        return Some(piper_link)
    }
    None
}