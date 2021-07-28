use std::path::{Path, PathBuf};

pub fn get_tmp_file(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../tmp")
        .join(name)
}
