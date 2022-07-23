use mini_redis::client;
use tokio_stream::StreamExt;

async fn publish() -> mini_redis::Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await?;

    client.publish("numbers", "1".into()).await?;
    client.publish("numbers", "2".into()).await?;
    client.publish("numbers", "three".into()).await?;
    client.publish("numbers", "4".into()).await?;
    client.publish("numbers", "five".into()).await?;
    Ok(())
}

async fn subscribe() -> mini_redis::Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await?;
    let subscribe = client.subscribe(vec!["numbers".to_string()]).await?;
    let message = subscribe.into_stream();
    tokio::pin!(message);

    while let Some(m) = message.next().await {
        println!("GOT: {:?}", m);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> mini_redis::Result<()> {
    tokio::spawn(async { publish().await });

    subscribe().await?;
    println!("DONE");

    Ok(())
}
