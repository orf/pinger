# pinger
[![Crates.io](https://img.shields.io/crates/v/pinger.svg)](https://crates.io/crates/pinger)
[![Run Tests](https://github.com/orf/pinger/workflows/Run%20Tests/badge.svg)](https://github.com/orf/pinger/action)

This is a small Rust library to execute `ping` and parse the output across different platforms.

## Install

`cargo add pinger`

## Usage

The `ping()` function is the main entrypoint to the library. It returns an asynchronous stream of `PingResult` values, 
which can be either a `Timeout` or a `Pong(f32)`. Below is a simple `ping` implementation:

```rust
use futures::pin_mut;
use futures::StreamExt;
use pinger::ping;

#[tokio::main]
async fn main() {
    let stream = ping("tomforb.es".to_string())
        .expect("Error pinging");
    pin_mut!(stream);
    while let Some(value) = stream.next().await {
        println!("{:?}", value);
    }
}
```

## Why?

Sending ICMP messages across platforms is complicated and on some platforms this also requires root access to do.  
Executing `ping` and parsing the output is a somewhat simple cross-platform way of doing this.
