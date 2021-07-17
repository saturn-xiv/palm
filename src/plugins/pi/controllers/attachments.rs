use std::fs::{create_dir_all, File};
use std::io::prelude::*;
use std::path::Path;

use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};

use super::super::super::super::Result;

pub async fn create<P: AsRef<Path>>(root: P, mut payload: Multipart) -> Result<()> {
    let root = root.as_ref();
    create_dir_all(root)?;
    while let Ok(Some(mut field)) = payload.try_next().await {
        if let Some(title) = field.content_disposition() {
            if let Some(title) = title.get_filename() {
                info!("receive file {}", title);
                let mut file = File::create(root.join(title))?;

                while let Some(chunk) = field.next().await {
                    let data = chunk?;
                    file.write_all(&data)?;
                }
            }
        }
    }
    Ok(())
}
