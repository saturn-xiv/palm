pub mod protocols;

use super::super::{Result, Thrift};

use self::protocols::{MarkdownSyncClient, TMarkdownSyncClient};

const SERVICE: &str = "markdown";

pub trait Markdown {
    fn to_html(&self, text: &str, sanitize: bool) -> Result<String>;
}

impl Markdown for Thrift {
    fn to_html(&self, text: &str, sanitize: bool) -> Result<String> {
        let (i_prot, o_prot) = self.open(SERVICE)?;
        let mut client: MarkdownSyncClient<_, _> = MarkdownSyncClient::new(i_prot, o_prot);
        let html = client.to_html(text.to_string(), sanitize)?;
        Ok(html)
    }
}
