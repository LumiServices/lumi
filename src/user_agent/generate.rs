use rand::seq::IndexedRandom;

const UA_PLATS: [&str; 10] = [
    "Windows NT 10.0; Win64; x64",
    "Windows NT 6.3; Win64; x64",
    "Macintosh; Intel Mac OS X 10_15_7",
    "Macintosh; Intel Mac OS X 14_0",
    "X11; Linux x86_64",
    "X11; Ubuntu; Linux x86_64",
    "Android 13; SM-G991B",
    "Android 14; Pixel 8",
    "iPhone; CPU iPhone OS 17_0 like Mac OS X",
    "iPad; CPU OS 17_0 like Mac OS X",
];

const UA_BROWSERS: [&str; 10] = [
    "AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36",
    "AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36",
    "AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
    "Gecko/20100101 Firefox/119.0",
    "Gecko/20100101 Firefox/118.0",
    "AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Safari/605.1.15",
    "AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.6 Safari/605.1.15",
    "AppleWebKit/537.36 (KHTML, like Gecko) Mobile Safari/537.36",
    "AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Mobile/15E148 Safari/604.1",
    "AppleWebKit/537.36 (KHTML, like Gecko) Edg/124.0.0.0 Safari/537.36",
];

#[allow(deprecated)] //fuck off
pub fn generate_ua() -> String {
    let platform = UA_PLATS.choose(&mut rand::thread_rng()).unwrap();
    let browser = UA_BROWSERS.choose(&mut rand::thread_rng()).unwrap();
    format!("Mozilla/5.0 ({}) {}", platform, browser)
}