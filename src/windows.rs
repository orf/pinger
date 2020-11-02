use crate::{Parser, PingResult};
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(?ix-u)time=(?P<time>\d+(?:\.\d+)?)ms").unwrap();
}

#[derive(Default)]
pub struct WindowsParser {}

impl Parser for WindowsParser {
    fn parse(&self, line: String) -> Option<PingResult> {
        if line.contains("timed out") || line.contains("failure") {
            return Some(PingResult::Timeout);
        }
        self.extract_regex(&RE, line)
    }
}
