use tokio::{io::{self, AsyncReadExt, AsyncWriteExt}, fs::File};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut f = File::open("foo.txt").await?;
    let mut buffer = [0; 10];

    let n = f.read(&mut buffer[..]).await?;

    println!("the bytes: {:?}", &buffer[..n]);
    // read all 
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).await?;
    println!("the bytes: {:?}", &buffer);
    // write
    let mut file = File::create("foo.txt").await?;
    let n = file.write(b"some bytes").await?;

    println!("Wrote the first {} bytes of 'some bytes'", n);
    // write all
    file.write_all(b"some bytes2").await?;

    let mut reader: &[u8] = b"hello";
    let mut file = File::create("foo.txt").await?;

    io::copy(&mut reader, &mut file).await?;

    Ok(())
}
