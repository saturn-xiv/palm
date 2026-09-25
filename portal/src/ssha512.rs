use data_encoding::BASE64;
use openssl::{hash::MessageDigest, sha::Sha512};

use super::random::bytes as random_bytes;

// https://mad9scientist.com/dovecot-password-creation-php/
// https://doc.dovecot.org/2.3/configuration_manual/howto/convert_password_schemes/
pub fn sign(password: &str, salt_len: usize) -> String {
    let salt = random_bytes(salt_len);
    let mut buf = Vec::new();
    let hash = {
        let mut hashed = Sha512::new();
        hashed.update(password.as_bytes());
        hashed.update(&salt);
        hashed.finish()
    };
    buf.extend_from_slice(&hash);
    buf.extend_from_slice(&salt);
    format!("{}{}", PREFIX, BASE64.encode(&buf))
}
pub fn verify(code: &str, password: &str) -> bool {
    let hash_size = {
        let it = MessageDigest::sha512();
        it.size()
    };
    if let Some(code) = code.strip_prefix(PREFIX)
        && let Ok(ref buf) = BASE64.decode(code.as_bytes())
        && buf.len() > hash_size
    {
        let tmp = {
            let salt = &buf[hash_size..];
            let mut hashed = Sha512::new();
            hashed.update(password.as_bytes());
            hashed.update(salt);
            hashed.finish()
        };
        return buf[0..hash_size] == tmp;
    }

    false
}

static PREFIX: &str = "{SSHA512}";
