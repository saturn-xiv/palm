use data_encoding::HEXLOWER;
use openssl::sha::Sha256;

// https://docs.gravatar.com/sdk/images/
pub fn image(email: &str) -> String {
    format!("https://gravatar.com/avatar/{}", hash(email))
}

// https://docs.gravatar.com/rest/hash/
pub fn hash(email: &str) -> String {
    let email = email.trim().to_lowercase();
    let buf = {
        let mut hashed = Sha256::new();
        hashed.update(email.as_bytes());
        hashed.finish()
    };

    HEXLOWER.encode(&buf)
}
