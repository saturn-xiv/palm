use rand::{
    Rng,
    distr::{Alphanumeric, SampleString},
};
use uuid::Uuid;

pub fn bytes(len: usize) -> Vec<u8> {
    let mut buf = vec![0; len];
    rand::rng().fill_bytes(&mut buf[..]);
    buf
}

pub fn alphanumeric(len: usize) -> String {
    Alphanumeric.sample_string(&mut rand::rng(), len)
}

pub fn uuid() -> String {
    return Uuid::new_v4().to_string();
}
