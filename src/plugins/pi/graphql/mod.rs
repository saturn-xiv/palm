pub mod dns;
pub mod network;
pub mod ntp;
pub mod ping;
pub mod vpn;

use std::process::Command;
use std::thread;
use std::time::Duration;

use bytesize::ByteSize;
use humantime::format_duration;
use juniper::GraphQLObject;
use nix::sys::{sysinfo::sysinfo, utsname::uname};

use super::super::super::{
    graphql::Session, jwt::Jwt, models::log::Dao as LogDao, models::user::Token,
    orm::Connection as Db, Result,
};

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
    pub fn new(ss: &Session, db: &Db, jwt: &Jwt) -> Result<Self> {
        ss.current_user(db, jwt)?;
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
    pub fn reboot(ss: &Session, db: &Db, jwt: &Jwt) -> Result<()> {
        let user = ss.current_user(db, jwt)?;
        LogDao::add(db, user.id, &ss.client_ip, "Reboot!")?;
        thread::sleep(Duration::from_secs(2));
        thread::spawn(|| {
            if let Err(e) = super::sys::reboot() {
                error!("{:?}", e);
            }
        });
        Ok(())
    }
    pub fn token(ss: &Session, db: &Db, jwt: &Jwt, years: i32) -> Result<String> {
        let user = ss.current_user(db, jwt)?;
        let (nbf, exp) = Jwt::years(years)?;
        let it = jwt.sum(
            None,
            &Token {
                uid: user.uid.clone(),
                sub: user.real_name,
                act: Token::SIGN_IN.to_string(),
                nbf,
                exp,
            },
        )?;
        Ok(it)
    }
    pub fn logs(ss: &Session, db: &Db, jwt: &Jwt, name: &str) -> Result<String> {
        ss.current_user(db, jwt)?;
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

    pub fn reset(ss: &Session, db: &Db, jwt: &Jwt) -> Result<()> {
        ss.current_user(db, jwt)?;
        // TODO
        todo!()
    }

    pub fn dump(ss: &Session, db: &Db, jwt: &Jwt) -> Result<()> {
        ss.current_user(db, jwt)?;
        // TODO
        todo!()
    }

    pub fn restore(ss: &Session, db: &Db, jwt: &Jwt) -> Result<()> {
        ss.current_user(db, jwt)?;
        // TODO
        Ok(())
    }

    pub fn upgrade(ss: &Session, db: &Db, jwt: &Jwt) -> Result<()> {
        ss.current_user(db, jwt)?;
        // TODO
        todo!()
    }
}
