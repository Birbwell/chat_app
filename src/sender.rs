use crate::message::Message;
use crate::prelude::*;
use serde_json::json;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use std::fs::OpenOptions;
use std::io::Write;

/// ### The main Sender loop.
///
/// Loops ad infinitum. It will handle input, parsing of input, and passing the data along to be sent.
pub async fn sender_loop(
    mut tx: tokio::sync::mpsc::Receiver<String>,
    user: String,
    ip: String,
    ssx: tokio::sync::mpsc::Sender<String>,
) -> Result<()> {
    // let mut conn = TcpStream::connect(ip).await?;
    loop {
        match tx.recv().await {
            Some(m) if m.len() > 0 => {
                let msg = Message::new(&user, &m);
                let msg_json = json!(msg).to_string();

                let Ok(_) = ssx.send(msg_json).await else {
                    return Ok(());
                };
            },
            _ => {},
        }
    }
}

#[cfg(test)]
mod tests {
    use tokio::net::{TcpListener, TcpStream};

    #[tokio::test]
    async fn binding_test() {
        let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
        let conn = TcpStream::connect("127.0.0.1:8080").await;
        _ = listener.accept().await.unwrap();
        println!("{conn:?}");
    }
}
