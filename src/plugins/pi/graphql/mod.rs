pub mod dns;
pub mod network;
pub mod ntp;
pub mod ping;
pub mod user;
pub mod vpn;

use std::process::Command;
use std::thread;
use std::time::Duration;

use bytesize::ByteSize;
use humantime::format_duration;
use juniper::GraphQLObject;
use nix::sys::{sysinfo::sysinfo, utsname::uname};

use super::super::super::{
    crypto::Aes, jwt::Jwt, orm::sqlite::Connection as Db, request::Token as Auth, Result,
};
use super::super::nut::models::user::Token;

use self::user::{CurrentUser, User};

#[derive(GraphQLObject)]
#[graphql(name = "Status")]
pub struct Status {
    uptime: String,
    uname: String,
    process: i32,
    load: Vec<String>,
    swap: String,
    ram: String,
}

impl Status {
    pub fn new(ss: &Auth, db: &Db, jwt: &Jwt, aes: &Aes) -> Result<Self> {
        ss.current_user(db, jwt, aes)?;
        let si = sysinfo()?;
        let un = uname();
        let load = si.load_average();
        Ok(Self {
            uptime: format_duration(si.uptime()).to_string(),
            uname: format!(
                "{} {} {} {} {}",
                un.sysname(),
                un.nodename(),
                un.machine(),
                un.release(),
                un.version()
            ),
            process: si.process_count() as i32,
            load: vec![
                format!("1 Minute: {:.2}", load.0),
                format!("5 Minutes: {:.2}", load.1),
                format!("15 Minutes: {:.2}", load.2),
            ],
            swap: format!(
                "{}/{}",
                ByteSize(si.swap_total() - si.swap_free()),
                ByteSize(si.swap_total())
            ),
            ram: format!(
                "{}/{}",
                ByteSize(si.ram_total() - si.ram_unused()),
                ByteSize(si.ram_total())
            ),
        })
    }
}

pub struct Pi {}

impl Pi {
    pub fn reboot(ss: &Auth, db: &Db, jwt: &Jwt, aes: &Aes) -> Result<()> {
        ss.current_user(db, jwt, aes)?;
        info!("reboot!");
        thread::sleep(Duration::from_secs(2));
        thread::spawn(|| {
            if let Err(e) = super::super::super::sys::reboot() {
                error!("{:?}", e);
            }
        });
        Ok(())
    }
    pub fn token(ss: &Auth, db: &Db, jwt: &Jwt, aes: &Aes, years: i32) -> Result<String> {
        let user = ss.current_user(db, jwt, aes)?;
        let (nbf, exp) = Jwt::years(years)?;
        let it = jwt.sum(
            None,
            &Token {
                uid: user.name,
                sub: User::KEY.to_string(),
                act: Token::SIGN_IN.to_string(),
                nbf,
                exp,
            },
        )?;
        Ok(it)
    }
    pub fn logs(ss: &Auth, db: &Db, jwt: &Jwt, aes: &Aes, name: &str) -> Result<String> {
        ss.current_user(db, jwt, aes)?;
        let output = Command::new("journalctl")
            .arg("-u")
            .arg(&name)
            .arg("-b")
            .output()?;
        let msg = if output.status.success() {
            String::from_utf8(output.stdout)?
        } else {
            format!("{:#?} {}", output.status, String::from_utf8(output.stderr)?)
        };
        Ok(msg)
    }

    pub fn reset(ss: &Auth, db: &Db, jwt: &Jwt, aes: &Aes) -> Result<()> {
        ss.current_user(db, jwt, aes)?;
        // TODO
        todo!()
    }

    pub fn dump(ss: &Auth, db: &Db, jwt: &Jwt, aes: &Aes) -> Result<()> {
        ss.current_user(db, jwt, aes)?;
        // TODO
        todo!()
    }

    pub fn restore(ss: &Auth, db: &Db, jwt: &Jwt, aes: &Aes) -> Result<()> {
        ss.current_user(db, jwt, aes)?;
        // TODO
        Ok(())
    }

    pub fn upgrade(ss: &Auth, db: &Db, jwt: &Jwt, aes: &Aes) -> Result<()> {
        ss.current_user(db, jwt, aes)?;
        // TODO
        todo!()
    }
}
