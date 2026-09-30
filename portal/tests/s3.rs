use std::path::PathBuf;

use portal::{models::attachment::Item as Attachment, s3::seaweedfs::Config};

#[tokio::test]
async fn aws_s3() {}

#[tokio::test]
async fn seaweedfs() {
    let config = Config::default();
    println!("###### {:?} ===", config);
    let cli = config.open();
    {
        let target = cli.assign().await.unwrap();
        println!("{:?}", target);
        let file = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("README.md");
        let res = cli.upload(&file, &target.url, &target.fid).await.unwrap();
        println!("{:?}", res);
        {
            let it = cli.lookup_volume_by_id(&target.fid).await.unwrap();
            println!("{:?}", it);
            assert!(it.locations.len() > 0);
        }
    }
}

#[test]
fn attachment_uid() {
    let it = Attachment {
        bucket: "bbb".to_string(),
        object: "ooo".to_string(),
        ..Default::default()
    };
    {
        let uid = it.uid();
        let (bucket, object) = Attachment::from_uid(&uid).unwrap();
        println!("{uid} => ({bucket},{object})");
        assert_eq!(it.bucket, bucket);
        assert_eq!(it.object, object);
    }
}
