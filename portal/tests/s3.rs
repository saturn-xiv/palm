use portal::models::attachment::Item as Attachment;

#[tokio::test]
async fn aws_s3() {}

#[tokio::test]
async fn seaweedfs() {}

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
