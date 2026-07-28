use hyacinth::loquat_v1::{Argon2SignRequest, Argon2VerifyRequest};

use super::super::{HttpError, Loquat, PasswordHashing, Result};

impl PasswordHashing for Loquat {
    async fn sign(&self, password: &str) -> Result<String> {
        let mut req = Argon2SignRequest::new();
        req.set_password(password);

        let res = self
            .argon2
            .sign(req)
            .await
            .map_err(|x| Box::<HttpError>::new(x.into()))?;
        Ok(res.hashed().to_string())
    }
    async fn verify(&self, hashed: &str, password: &str) -> Result<()> {
        let mut req = Argon2VerifyRequest::new();
        req.set_hashed(hashed);
        req.set_password(password);

        self.argon2
            .verify(req)
            .await
            .map_err(|x| Box::<HttpError>::new(x.into()))?;
        Ok(())
    }
}
