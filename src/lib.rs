use anyhow::Result;
use async_stream::stream;
use futures::stream::Stream;
use os_info::Type;
use regex::Regex;
use std::pin::Pin;
use std::process::Stdio;
use thiserror::Error;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use std::time::Duration;

#[macro_use]
extern crate lazy_static;

pub mod linux;
pub mod macos;
#[cfg(test)]
mod test;
pub mod windows;

pub trait Pinger: Default {
    fn start<P>(&self, target: String) -> Result<Pin<Box<dyn Stream<Item = PingResult>>>>
    where
        P: Parser,
    {
        let mut child = Command::new("ping")
            .args(self.ping_args(target))
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .kill_on_drop(true)
            .spawn()?;

        let stream = stream! {
            let parser = P::default();
            let stdout = child.stdout.take().expect("child did not have a stdout");
            let mut reader = BufReader::new(stdout).lines();
            while let Some(line) = reader.next_line().await.expect("next_line() failed") {
                if let Some(result) = parser.parse(line) {
                    yield result;
                }
            }
        };
        Ok(Box::pin(stream))
    }

    fn ping_args(&self, target: String) -> Vec<String> {
        return vec![target];
    }
}

// Default empty implementation of a pinger.
#[derive(Default)]
pub struct SimplePinger {}

impl Pinger for SimplePinger {}

pub trait Parser: Default {
    fn parse(&self, line: String) -> Option<PingResult>;

    fn extract_regex(&self, regex: &Regex, line: String) -> Option<PingResult> {
        let cap = regex.captures(&line)?;
        let time = cap
            .name("time")
            .expect("No capture group named 'time'")
            .as_str()
            .parse::<f32>()
            .expect("time cannot be parsed as f32");
        Some(PingResult::Pong(Duration::from_micros((time * 100f32) as u64)))
    }
}

#[derive(Debug)]
pub enum PingResult {
    Pong(Duration),
    Timeout,
}

#[derive(Error, Debug)]
pub enum PingError {
    #[error("Unsupported OS {0}")]
    UnsupportedOS(String),
}

pub fn ping(addr: String) -> Result<impl Stream<Item = PingResult>> {
    let os_type = os_info::get().os_type();
    match os_type {
        Type::Windows => {
            let p = SimplePinger::default();
            p.start::<windows::WindowsParser>(addr)
        }
        Type::Linux | Type::Debian | Type::Ubuntu | Type::Alpine => {
            let p = linux::LinuxPinger::default();
            p.start::<linux::LinuxParser>(addr)
        }
        Type::Macos => {
            let p = SimplePinger::default();
            p.start::<macos::MacOSParser>(addr)
        }
        _ => Err(PingError::UnsupportedOS(os_type.to_string()).into()),
    }
}
