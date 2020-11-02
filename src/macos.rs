use crate::{Parser, PingResult};
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(
        r"(?ix-u)
    \s?[0-9]*                            # Bytes of data
    \sbytes\sfrom\s                      # bytes from
    \d+\.\d+\.\d+\.\d+:
    \s+icmp_seq=\d+                      # icmp_seq
    \s+ttl=\d+                           # ttl
    \s+time=(?:(?P<time>[0-9\.]+)\s+ms)  # capture time"
    )
    .unwrap();
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
