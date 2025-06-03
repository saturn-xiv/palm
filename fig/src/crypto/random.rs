use rand::{distr::Alphanumeric, prelude::*, rng};
use uuid::Uuid;

pub fn bytes(l: usize) -> Vec<u8> {
    let mut rng = rng();
    (0..l).map(|_| rng.random::<u8>()).collect()
}

pub fn string(l: usize) -> String {
    let mut rng = rng();
    std::iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(l)
        .collect()
}

pub fn uuid() -> String {
    Uuid::new_v4().to_string()
}
