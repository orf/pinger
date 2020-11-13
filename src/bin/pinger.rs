use pinger::{ping, PingResult};

fn main() {
    let host = std::env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("tomforb.es"));
    let stream = ping(host).expect("Error pinging");
    for message in stream.iter().take(10) {
        match message {
            PingResult::Pong(duration) => println!("{:?}", duration),
            PingResult::Timeout => println!("Timeout!"),
        }
    }
}
