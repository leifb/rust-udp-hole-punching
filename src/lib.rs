use bincode::{Encode, Decode};

#[derive(Encode, Decode, Debug)]
pub enum Message {
    String(String),
}