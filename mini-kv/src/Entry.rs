use std::io;

use serde_repr::{Deserialize_repr, Serialize_repr};

static USIZE_LEN: usize = std::mem::size_of::<usize>();
static ENTRY_HEAD_LEN: usize = USIZE_LEN * 2 + std::mem::size_of::<CmdKind>();

pub struct Entry {
    key_len: usize,
    value_len: usize,
    kind: CmdKind,
    key: String,
    value: String,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum CmdKind {
    PUT = 1,
    DEL = 2,
}

impl Entry {
    pub fn size(&self) -> usize {
        ENTRY_HEAD_LEN + self.key_len + self.value_len
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buf = vec![0; self.size()];
        // encode key len
        buf[0..USIZE_LEN].copy_from_slice(&self.key_len.to_be_bytes());
        // encode value len
        buf[USIZE_LEN..USIZE_LEN * 2].copy_from_slice(&self.value_len.to_be_bytes());
        // encode kind
        buf[USIZE_LEN * 2..ENTRY_HEAD_LEN]
            .copy_from_slice(bincode::serialize(&self.kind).unwrap().as_slice());

        // encode key
        buf[ENTRY_HEAD_LEN..ENTRY_HEAD_LEN + self.key_len].copy_from_slice(self.key.as_bytes());
        // encode value
        buf[ENTRY_HEAD_LEN + self.key_len..].copy_from_slice(self.value.as_bytes());

        buf
    }

    pub fn decode(b: Vec<u8>) -> Result<Entry, io::Error> {
        let key_len = usize::from_be_bytes(b[0..USIZE_LEN].try_into()?);
        let value_len = usize::from_be_bytes(b[USIZE_LEN..USIZE_LEN*2].try_into()?);
        let kind: CmdKind = bincode::deserialize(&b[USIZE_LEN *2..ENTRY_HEAD_LEN])?;

        None
    }
}
