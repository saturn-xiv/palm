pub mod aws;
pub mod seaweedfs;

use std::future::Future;
use std::path::Path;

use super::Result;

pub trait Provider {
    fn upload<P: AsRef<Path>>(file: P) -> impl Future<Output = Result<()>> + Send;
    fn download(id: &str) -> impl Future<Output = Result<Vec<u8>>> + Send;
}
