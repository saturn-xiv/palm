pub mod aes;
pub mod hmac;
pub mod jwt;
pub mod sha1;
pub mod ssha512;

use openssl::hash::{hash, MessageDigest};

use super::Result;

// https://en.gravatar.com/site/implement/hash/
pub fn gravatar<S: AsRef<str>>(email: &S) -> Result<String> {
    let id = hash(
        MessageDigest::md5(),
        email.as_ref().to_lowercase().trim().as_bytes(),
    )?;
    let it = format!(
        "https://www.gravatar.com/avatar/{}.png",
        id.to_vec()
            .iter()
            .map(|x| format!("{:02x}", *x))
            .collect::<Vec::<_>>()
            .join("")
    );
    Ok(it)
}
