use super::Result;

pub struct Captcha {
    pub text: String,
    pub height: u32,
    pub gap: u32,
}

impl Captcha {
    pub fn png(&self) -> Result<Vec<u8>> {
        // TODO
        todo!()
    }
}
