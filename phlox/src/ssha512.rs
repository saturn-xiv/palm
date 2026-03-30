use data_encoding::BASE64_NOPAD;
use hyper::StatusCode;
use sha2::{Digest, Sha512};

use super::{HttpError, Result, random};

pub fn sign<P: AsRef<str>>(password: P, salt_len: usize) -> Result<String> {
    let salt = random::bytes(salt_len);
    let password = password.as_ref();
    sum(password, &salt)
}

pub fn verify<P: AsRef<str>, H: AsRef<str>>(hash: H, password: P) -> Result<()> {
    let hash = hash.as_ref();
    let tmp = {
        let it = hash.strip_prefix(HEADER).ok_or_else(|| {
            HttpError(
                StatusCode::BAD_REQUEST,
                Some("invalid ssha512 header".to_string()),
            )
        })?;
        let mut buf = vec![0; BASE64_NOPAD.decode_len(it.len())?];
        BASE64_NOPAD
            .decode_mut(it.as_bytes(), &mut buf)
            .map_err(|x| x.error)?;
        sum(password, &buf[Sha512::output_size()..])?
    };
    if hash != tmp {
        return Err(Box::new(HttpError(
            StatusCode::FORBIDDEN,
            Some("password isn't matched".to_string()),
        )));
    }
    Ok(())
}

// https://mad9scientist.com/dovecot-password-creation-php/
fn sum<P: AsRef<str>>(message: P, salt: &[u8]) -> Result<String> {
    let mut buf = Vec::new();
    let hash = {
        let message = message.as_ref();
        let mut it = Sha512::new();
        it.update(message);
        it.update(salt);
        it.finalize()
    };
    buf.extend(&hash);
    buf.extend(salt);

    Ok(format!("{}{}", HEADER, BASE64_NOPAD.encode(&buf)))
}

const HEADER: &str = "{SSHA512}";
