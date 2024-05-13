pub mod protocols;

use chrono::Duration;
use thrift::Result as ThriftResult;

use super::Thrift;

use self::protocols::{S3SyncClient, TS3SyncClient};

const S3_SERVICE_NAME: &str = "github.com/saturn-xiv/palm/jasmine/services/v1/S3";

pub trait S3 {
    fn create_bucket(&self, name: &str, public: bool, expiration_days: i32) -> ThriftResult<()>;
    fn upload_file(&self, bucket: &str, object: &str, ttl: Duration) -> ThriftResult<String>;
    fn get_presigned_url(
        &self,
        bucket: &str,
        object: &str,
        title: &str,
        ttl: Duration,
    ) -> ThriftResult<String>;
    fn get_permanent_url(&self, bucket: &str, object: &str) -> ThriftResult<String>;
}
impl S3 for Thrift {
    fn create_bucket(&self, name: &str, public: bool, expiration_days: i32) -> ThriftResult<()> {
        let (i_prot, o_prot) = self.open(S3_SERVICE_NAME)?;
        let mut client: S3SyncClient<_, _> = S3SyncClient::new(i_prot, o_prot);
        client.create_bucket(name.to_string(), public, expiration_days)?;
        Ok(())
    }
    fn upload_file(&self, bucket: &str, object: &str, ttl: Duration) -> ThriftResult<String> {
        let (i_prot, o_prot) = self.open(S3_SERVICE_NAME)?;
        let mut client: S3SyncClient<_, _> = S3SyncClient::new(i_prot, o_prot);
        let url = client.upload_file(
            bucket.to_string(),
            object.to_string(),
            ttl.num_seconds() as i32,
        )?;
        Ok(url)
    }
    fn get_presigned_url(
        &self,
        bucket: &str,
        object: &str,
        title: &str,
        ttl: Duration,
    ) -> ThriftResult<String> {
        let (i_prot, o_prot) = self.open(S3_SERVICE_NAME)?;
        let mut client: S3SyncClient<_, _> = S3SyncClient::new(i_prot, o_prot);
        let url = client.get_presigned_url(
            bucket.to_string(),
            object.to_string(),
            title.to_string(),
            ttl.num_seconds() as i32,
        )?;
        Ok(url)
    }
    fn get_permanent_url(&self, bucket: &str, object: &str) -> ThriftResult<String> {
        let (i_prot, o_prot) = self.open(S3_SERVICE_NAME)?;
        let mut client: S3SyncClient<_, _> = S3SyncClient::new(i_prot, o_prot);
        let url = client.get_permanent_url(bucket.to_string(), object.to_string())?;
        Ok(url)
    }
}
