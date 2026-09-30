pub mod aws;
pub mod seaweedfs;

use std::fs;
use std::future::Future;
use std::io::Result as IoResult;
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
    //
    fn file_size<P: AsRef<Path>>(file: P) -> IoResult<u64> {
        let file = file.as_ref();
        let size = {
            let md = fs::metadata(file)?;
            md.len()
        };
        Ok(size)
    }
}
