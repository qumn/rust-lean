use mini_redis::Result;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379").await?;
    while let Ok((mut socket, _)) = listener.accept().await {
        tokio::spawn(async move {
            let mut buf = vec![0; 1024];
            loop {
                match socket.read(&mut buf).await {
                    Ok(0) => return,
                    Ok(n) => {
                        println!("GOT: {:?}", &buf[..n]);

                        if socket.write_all(&buf[..n]).await.is_err() {
                            return;
                        }
                    }
                    Err(_) => {
                        return;
                    }
                }
            }
            // let (mut rd, mut wr) = io::split(socket);
            // tokio::spawn(async move {
            //     io::copy(&mut rd, &mut wr).await.unwrap();
            // });
        });
    }
    Ok(())
}
