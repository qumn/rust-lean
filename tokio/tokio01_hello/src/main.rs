use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use bytes::Bytes;
use log::trace;
use mini_redis::{Command, Connection, Frame, Result};
use tokio::net::{TcpListener, TcpStream};

type Db = Arc<Vec<Mutex<HashMap<String, bytes::Bytes>>>>;
static SHARE_NUMBER: usize = 3;

fn new_share_db() -> Db {
    let mut share_db = Vec::with_capacity(SHARE_NUMBER);
    for _ in 0..SHARE_NUMBER {
        share_db.push(Mutex::new(HashMap::new()));
    }
    Arc::new(share_db)
}

fn set_value(db: &Db, key: &str, value: &Bytes) {
    let mut db = db[key.len() % SHARE_NUMBER].lock().unwrap();
    trace!("Set a key:{:?}, value: {:?}", key, value);
    db.insert(key.to_string(), value.clone());
}

fn get_value(db: &Db, key: &str) -> Option<Bytes> {
    let db = db[key.len() % SHARE_NUMBER].lock().unwrap();
    db.get(key).map(|k| k.clone()) // because bytes like a arc<mutex>, the clone no performance loss

}

#[tokio::main]
async fn main() -> Result<()> {
    // start log
    env_logger::init();
    let db = new_share_db();

    let listener = TcpListener::bind("127.0.0.1:6379").await?;

    loop {
        let (socket, _) = listener.accept().await?;
        let db = db.clone();
        tokio::spawn(async move {
            process(db, socket).await.unwrap();
        });
    }
}

async fn process(db: Db, socket: TcpStream) -> Result<()> {
    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await? {
        trace!("GOT a frame: {:?}", frame);
        let response = match Command::from_frame(frame)? {
            Command::Set(cmd) => {
                set_value(&db, cmd.key(), cmd.value());
                Frame::Simple("OK".to_string())
            }
            Command::Get(cmd) => {
                if let Some(value) = get_value(&db, cmd.key()) {
                    Frame::Bulk(value.clone().into())
                } else {
                    trace!("value not found");
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        connection.write_frame(&response).await?;
    }

    Ok(())
}
