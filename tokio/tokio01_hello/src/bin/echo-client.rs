use mini_redis::Result;
use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

#[tokio::main]
async fn main() -> Result<()> {
    let connect = TcpStream::connect("127.0.0.1:6379").await?;
    let (mut rd, mut wr) = io::split(connect);

    //let n = rd.read(&mut buf).await?;
    tokio::spawn(async move {
        wr.write_all(b"hello\r\n").await?;
        println!("WROTE one message");
        wr.write_all(b"word\r\n").await?;
        println!("WROTE two message");
        Ok::<_, io::Error>(())
    });

    let mut buffer = vec![0; 128];
    loop {
        println!("prepare to read");
        let n = rd.read(&mut buffer).await?;
        if n == 0 {
            break;
        }
        println!("GOT {:?}", &buffer[..n]);
    }

    // let socket = TcpStream::connect("127.0.0.1:6379").await?;
    // let (mut rd, mut wr) = io::split(socket);

    // // Write data in the background
    // tokio::spawn(async move {
    //     wr.write_all(b"hello\r\n").await?;
    //     wr.write_all(b"world\r\n").await?;

    //     // Sometimes, the rust type inferencer needs
    //     // a little help
    //     Ok::<_, io::Error>(())
    // });

    // let mut buf = vec![0; 128];

    // loop {
    //     let n = rd.read(&mut buf).await?;

    //     if n == 0 {
    //         break;
    //     }

    //     println!("GOT {:?}", &buf[..n]);
    // }

    Ok(())
}
