use rand::{
    RngExt,
    distr::{Alphanumeric, SampleString},
};

pub fn bytes(len: usize) -> Vec<u8> {
    let mut buf = vec![0; len];
    let mut rng = rand::rng();
    rng.fill(&mut buf);
    buf
}

pub fn alphanumeric(len: usize) -> String {
    let mut rng = rand::rng();
    Alphanumeric.sample_string(&mut rng, len)
}
