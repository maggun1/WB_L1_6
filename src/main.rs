use flume::{Sender, Receiver};
use tokio::signal;

async fn producer(tx: Sender<usize>) {
    let mut counter = 0;

    loop {
        if tx.send_async(counter).await.is_err() {
            println!("Receiver dropped. Stopping producer.");
            break;
        }
        println!("Produced: {}", counter);
        counter += 1;
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}

async fn consumer(rx: Receiver<usize>) {
    while let Ok(value) = rx.recv_async().await {
        println!("Consumed: {}", value);
    }
    println!("Channel closed. Stopping consumer.");
}

#[tokio::main]
async fn main() {
    let n: u64 = 5;

    let (tx, rx) = flume::unbounded();

    let producer_handle = tokio::spawn(producer(tx));
    let consumer_handle = tokio::spawn(consumer(rx));

    tokio::time::sleep(tokio::time::Duration::from_secs(n)).await;
    println!("Time is up! Shutting down...");

    producer_handle.abort();
    consumer_handle.await.unwrap();
}
