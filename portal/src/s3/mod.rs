pub mod aws;
pub mod seaweedfs;

use std::future::Future;
use std::path::Path;

use super::Result;

pub trait Provider {
    fn upload<P: AsRef<Path>>(
        &self,
        file: P,
        bucket: &str,
        object: &str,
    ) -> impl Future<Output = Result<()>>;
    // fn download(id: &str) -> impl Future<Output = Result<Vec<u8>>>;
}
