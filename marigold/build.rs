use std::env;
use std::fs::{File, read_dir};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Some(work_dir) = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?).parent() {
        let folders: Vec<PathBuf> = [
            &PathBuf::from("camellia").join("src").join("main"),
            &PathBuf::from("daisy"),
            &PathBuf::from("tulip"),
        ]
        .iter()
        .map(|x| work_dir.join(x).join("proto"))
        .collect();
        compile_protos(&folders)?;
    }
    {
        let out_dir = env::var("OUT_DIR")?;
        let git_version = shell(
            Command::new("git")
                .arg("describe")
                .arg("--tags")
                .arg("--always")
                .arg("--first-parent")
                .arg("--dirty"),
        )?;
        let build_time = shell(Command::new("date").arg("-u").arg("-R"))?;

        let dest_path = Path::new(&out_dir).join("env.rs");
        let mut fd = File::create(dest_path)?;

        writeln!(fd, r#"pub const GIT_VERSION: &str = "{git_version}";"#)?;
        writeln!(fd, r#"pub const BUILD_TIME: &str = "{build_time}";"#)?;
    }

    Ok(())
}

fn shell(cmd: &mut Command) -> Result<String, Box<dyn std::error::Error>> {
    let it = cmd.output()?;
    let it = std::str::from_utf8(&it.stdout)?;
    let it = it.trim();
    Ok(it.to_string())
}

fn compile_protos(includes: &[PathBuf]) -> Result<(), Box<dyn std::error::Error>> {
    // let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let mut files = Vec::new();
    for jt in includes {
        for it in read_dir(jt)? {
            let it = it?;
            let it = it.path();
            if let Some(ext) = it.extension()
                && ext == "proto"
                && let Some(name) = it.file_name()
                && let Some(name) = name.to_str()
            {
                files.push(name.to_string());
            }
        }
    }

    grpc_protobuf_build::CodeGen::new()
        .includes(includes)
        .inputs(files)
        .compile()?;
    Ok(())
}
