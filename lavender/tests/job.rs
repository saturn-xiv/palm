use std::collections::BTreeMap;

use lavender::models::job::{Arg, Item as Job};

#[test]
fn job_sample() {
    let it = {
        let mut args = BTreeMap::new();
        args.insert(
            "t".to_string(),
            Arg::Text {
                label: "Text Message".to_string(),
            },
        );
        args.insert(
            "s".to_string(),
            Arg::Select {
                label: "Select Message".to_string(),
                options: {
                    let mut items = BTreeMap::new();
                    items.insert("value-1".to_string(), "Value 1".to_string());
                    items.insert("value-3".to_string(), "Value 2".to_string());
                    items
                },
            },
        );
        args.insert(
            "g".to_string(),
            Arg::Git {
                label: "Git Message".to_string(),
                url: "https://github.com/saturn-xiv/palm.git".to_string(),
                branch: "development".to_string(),
            },
        );
        Job {
            version: "v2026.9.12".to_string(),
            command: "/bin/bash run.sh".to_string(),
            description: "Echo your message".to_string(),
            args: args,
        }
    };

    let buf = toml::to_string(&it).unwrap();
    println!("{buf}");
}
