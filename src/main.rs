use futures::channel::mpsc::channel;
use futures::future::join;
use futures::prelude::*;

#[async_std::main]
async fn main() {
    let (mut send, incoming) = channel(0);
    async_std::task::spawn(async move {
        let mut buf = String::new();
        while async_std::io::stdin().read_line(&mut buf).await.is_ok() {
            send.send(serde_json::from_str(&buf).unwrap()).await.unwrap();
            buf.clear();
        }
    });

    let (outgoing, mut recv) = channel(0);
    let output = async move {
        while let Some(msg) = recv.next().await {
            serde_json::to_writer(std::io::stdout(), &msg).unwrap();
            println!();
        }
    };

    join(output, dellacherie::main(incoming, outgoing)).await;
}
