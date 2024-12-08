pub mod bills;
pub mod show;

pub const AUDIENCE: &str = "ledger.show";

pub fn home_url(token: &str) -> String {
    format!("/accounting/ledgers/{token}/")
}
