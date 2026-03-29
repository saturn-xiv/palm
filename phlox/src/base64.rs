use data_encoding::{BASE64URL_NOPAD, DecodeError};

pub fn encode(buf: &[u8]) -> String {
    BASE64URL_NOPAD.encode(buf)
}

pub fn decode(str: &str) -> Result<Vec<u8>, DecodeError> {
    let mut buf = vec![0; BASE64URL_NOPAD.decode_len(str.len())?];
    BASE64URL_NOPAD
        .decode_mut(str.as_bytes(), &mut buf)
        .map_err(|x| x.error)?;
    Ok(buf)
}
