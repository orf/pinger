use pinger::{ping, PingResult};

fn main() {
    let stream = ping("tomforb.es".to_string()).expect("Error pinging");
    for message in stream.iter().take(10) {
        match message {
            PingResult::Pong(duration) => println!("{:?}", duration),
            PingResult::Timeout => println!("Timeout!")
        }
    }
}
