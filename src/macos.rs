use crate::{Parser, PingResult, Pinger};
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(
        r"time=(?:(?P<time>[0-9\.]+)\s+ms)"
    )
    .unwrap();
}

#[derive(Default)]
pub struct MacOSPinger {}

impl Pinger for MacOSPinger {
    fn ping_args(&self, target: String) -> Vec<String> {
        vec!["-i0.2".to_string(), target]
    }
}

#[derive(Default)]
pub struct MacOSParser {}

impl Parser for MacOSParser {
    fn parse(&self, line: String) -> Option<PingResult> {
        if line.starts_with("PING ") {
            return None;
        }
        if line.starts_with("Request timeout") {
            return Some(PingResult::Timeout);
        }
        self.extract_regex(&RE, line)
    }
}
