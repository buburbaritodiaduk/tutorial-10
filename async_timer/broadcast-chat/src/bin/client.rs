use futures_util::{SinkExt, StreamExt};
use http::Uri;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_websockets::{ClientBuilder, Message};

#[tokio::main]
async fn main() -> Result<(), tokio_websockets::Error> {
    let (mut ws_stream, _) = ClientBuilder::from_uri(Uri::from_static("ws://127.0.0.1:2000"))
        .connect()
        .await?;

    println!("Connected to the chat server! Type your message:");

    let stdin = tokio::io::stdin();
    let mut stdin = BufReader::new(stdin).lines();

    loop {
        tokio::select! {
            incoming = ws_stream.next() => {
                if let Some(Ok(msg)) = incoming {
                    if msg.is_text() {
                        println!("From server: {}", msg.as_text().unwrap());
                    }
                }
            }
            line = stdin.next_line() => {
                if let Ok(Some(text)) = line {
                    ws_stream.send(Message::text(text)).await?;
                }
            }
        }
    }
}