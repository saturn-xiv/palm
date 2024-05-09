use std::fs::read_dir;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    for it in read_dir("protocols")? {
        let it = it?;
        let it = it.path();
        if let Some(ext) = it.extension() {
            if ext == "proto" {
                tonic_build::compile_protos(&it)?;
            }
        }
    }
    Ok(())
}
