use futures::pin_mut;
use futures::StreamExt;
use pinger::ping;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let stream = ping("tomforb.es".to_string())
        .expect("Error pinging");
    pin_mut!(stream);
    while let Some(value) = stream.next().await {
        println!("{:?}", value);
    }
}
