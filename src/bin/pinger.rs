use pinger::ping;

fn main() {
    let stream = ping("tomforb.es".to_string()).expect("Error pinging");
    for message in stream.iter().take(10) {
        println!("{:?}", message);
    }
}
