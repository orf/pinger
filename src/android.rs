use crate::{Parser, PingResult, Pinger};
use regex::Regex;

#[derive(Default)]
pub struct AndroidPinger {}

impl Pinger for AndroidPinger {
    fn ping_args(&self, target: String) -> Vec<String> {
        vec!["-i0.2".to_string(), target]
    }
}

lazy_static! {
    static ref ANDROID_RE: Regex = Regex::new(r"(?i-u)time=(?P<time>\d+(?:\.\d+)?) *ms").unwrap();
}

#[derive(Default)]
pub struct AndroidParser {}

impl Parser for AndroidParser {
    fn parse(&self, line: String) -> Option<PingResult> {
        if line.starts_with("64 bytes from") {
            return self.extract_regex(&ANDROID_RE, line);
        } else if line.starts_with("no answer yet") {
            return Some(PingResult::Timeout);
        }
        None
    }
}
