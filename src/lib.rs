use anyhow::Result;
use os_info::Type;
use regex::Regex;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use thiserror::Error;

#[macro_use]
extern crate lazy_static;

pub mod linux;
pub mod macos;
#[cfg(windows)]
pub mod windows;

#[cfg(test)]
mod test;

pub trait Pinger: Default {
    fn start<P>(&self, target: String) -> Result<mpsc::Receiver<PingResult>>
    where
        P: Parser,
    {
        let (tx, rx) = mpsc::channel();
        let args = self.ping_args(target);

        thread::spawn(move || {
            let mut child = Command::new("ping")
                .args(args)
                .stdout(Stdio::piped())
                .stderr(Stdio::null())
                .spawn()
                .expect("Failed to run ping");
            let parser = P::default();
            let stdout = child.stdout.take().expect("child did not have a stdout");
            let reader = BufReader::new(stdout).lines();
            for line in reader {
                match line {
                    Ok(msg) => {
                        if let Some(result) = parser.parse(msg) {
                            if tx.send(result).is_err() {
                                break;
                            }
                        }
                    }
                    Err(_) => break,
                }
            }
        });

        Ok(rx)
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
        Some(PingResult::Pong(Duration::from_micros(
            (time * 1000f32) as u64,
        )))
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
    #[error("Invalid or unresolvable hostname {0}")]
    HostnameError(String),
}

pub fn ping(addr: String) -> Result<mpsc::Receiver<PingResult>> {
    let os_type = os_info::get().os_type();
    match os_type {
        #[cfg(windows)]
        Type::Windows => {
            let p = windows::WindowsPinger::default();
            p.start::<windows::WindowsParser>(addr)
        }
        Type::Amazon
        | Type::Arch
        | Type::CentOS
        | Type::Debian
        | Type::EndeavourOS
        | Type::Fedora
        | Type::Linux
        | Type::Manjaro
        | Type::Mint
        | Type::openSUSE
        | Type::OracleLinux
        | Type::Redhat
        | Type::RedHatEnterprise
        | Type::SUSE
        | Type::Ubuntu
        | Type::Pop
        | Type::Solus => {
            let p = linux::LinuxPinger::default();
            p.start::<linux::LinuxParser>(addr)
        }
        Type::Macos => {
            let p = macos::MacOSPinger::default();
            p.start::<macos::MacOSParser>(addr)
        }
        _ => Err(PingError::UnsupportedOS(os_type.to_string()).into()),
    }
}
