use std::path::PathBuf;

use portal::git::Host;

#[test]
fn git() {
    let repo = {
        let it = Host::Http {
            url: "https://github.com/saturn-xiv/pansy.git".to_string(),
            account: None,
        };
        it.open(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("tmp")
                .join("pansy.git"),
            "cpp",
        )
        .unwrap()
    };

    for it in repo.commit_logs().unwrap() {
        println!(
            "{} {}<{}> {} {}",
            it.id, it.created_at, it.username, it.email, it.message
        )
    }
}
