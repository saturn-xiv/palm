use std::fs::{remove_file, File, OpenOptions};
use std::io::prelude::*;
use std::ops::Deref;
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Component, Path};
use std::process::Command;
use std::sync::Arc;

use chrono::{DateTime, Local};
use chrono_tz::Tz;
use eui48::MacAddress;
use serde::{Deserialize, Serialize};
use tonic::{Request, Response, Status};
use validator::Validate;

use super::super::super::super::{
    auth::services::Session,
    crypto::Aes,
    jwt::Jwt,
    ntp::Response as NtpResponse,
    orm::sqlite::{Connection as Db, Pool as DbPool},
    sys::network::{
        ip4 as get_ip4, is_on, mac as get_mac,
        systemd::{Dhcp, Ip, Static, Wifi, Wpa},
    },
    GrpcResult, Result,
};
use super::super::{
    models::settings::Dao as SettingDao,
    v1::{
        network_profile, os_server::Os, status_response, DnsRequest, LinesResponse, LogsRequest,
        NetworkProfile, NtpProfile, PingRequest, RestoreRequest, StatusResponse, VpnProfile,
    },
};
use super::user::CurrentUser;

pub struct Service {
    pub db: DbPool,
    pub jwt: Arc<Jwt>,
    pub aes: Arc<Aes>,
}

#[tonic::async_trait]
impl Os for Service {
    async fn logs(&self, req: Request<LogsRequest>) -> GrpcResult<LinesResponse> {
        current_pi_user!(self, &req);
        let req = req.into_inner();
        let output = try_grpc!(
            Command::new("journalctl")
                .arg("-u")
                .arg(&req.name)
                .arg("-b")
                .output(),
            Status::invalid_argument
        )?;

        if !output.status.success() {
            return Err(Status::internal(format!(
                "{:#?} {}",
                output.status,
                try_grpc!(String::from_utf8(output.stderr))?
            )));
        }
        let out = try_grpc!(String::from_utf8(output.stdout))?;
        let lines: Vec<&str> = out.split('\n').collect();

        Ok(Response::new(LinesResponse {
            messages: lines.iter().map(|x| x.to_string()).collect(),
        }))
    }

    async fn status(&self, req: Request<()>) -> GrpcResult<StatusResponse> {
        current_pi_user!(self, &req);
        let si = try_grpc!(nix::sys::sysinfo::sysinfo())?;
        let un = nix::sys::utsname::uname();
        let load = si.load_average();
        Ok(Response::new(StatusResponse {
            uptime: Some(si.uptime().into()),
            uname: Some(status_response::Uname {
                sys: un.sysname().to_string(),
                node: un.nodename().to_string(),
                machine: un.machine().to_string(),
                release: un.release().to_string(),
                version: un.version().to_string(),
            }),
            process: si.process_count() as u32,
            load: Some(status_response::Load {
                one: load.0,
                five: load.1,
                fifteen: load.2,
            }),
            swap: Some(status_response::Range {
                total: si.swap_total(),
                free: si.swap_free(),
            }),
            ram: Some(status_response::Range {
                total: si.ram_total(),
                free: si.ram_total(),
            }),
            versions: Vec::new(),
        }))
    }

    async fn reboot(&self, req: Request<()>) -> GrpcResult<()> {
        current_pi_user!(self, &req);
        try_grpc!(super::super::super::super::sys::reboot())?;
        Ok(Response::new(()))
    }
    async fn reset(&self, req: Request<()>) -> GrpcResult<()> {
        current_pi_user!(self, &req);
        // TODO
        Ok(Response::new(()))
    }
    async fn dump(&self, req: Request<()>) -> GrpcResult<()> {
        current_pi_user!(self, &req);
        // TODO
        Ok(Response::new(()))
    }
    async fn restore(&self, req: Request<RestoreRequest>) -> GrpcResult<()> {
        current_pi_user!(self, &req);
        // TODO
        Ok(Response::new(()))
    }

    async fn ping(&self, req: Request<PingRequest>) -> GrpcResult<LinesResponse> {
        current_pi_user!(self, &req);
        let req = req.into_inner();
        let form = Ping { host: req.host };
        let out = try_grpc!(form.execute())?;
        let lines: Vec<&str> = out.split('\n').collect();

        Ok(Response::new(LinesResponse {
            messages: lines.iter().map(|x| x.to_string()).collect(),
        }))
    }
    async fn dns(&self, req: Request<DnsRequest>) -> GrpcResult<LinesResponse> {
        current_pi_user!(self, &req);
        let req = req.into_inner();
        let form = Dns {
            server: req.server.clone(),
            host: req.host,
        };
        let out = try_grpc!(form.execute())?;
        let lines: Vec<&str> = out.split('\n').collect();

        Ok(Response::new(LinesResponse {
            messages: lines.iter().map(|x| x.to_string()).collect(),
        }))
    }

    async fn get_network(&self, req: Request<()>) -> GrpcResult<NetworkProfile> {
        current_pi_user!(self, &req);
        let db = try_grpc!(self.db.get())?;
        let db = db.deref();
        let aes = self.aes.deref();
        let form: Network = SettingDao::get(db, aes, Network::KEY).unwrap_or_default();
        Ok(Response::new(NetworkProfile {
            eth: Some(network_profile::Eth {
                name: form.eth.name.clone(),
                ip: Some(match form.eth.ip {
                    Ip::Dhcp => network_profile::eth::Ip::Dhcp(true),
                    Ip::Static {
                        address,
                        netmask,
                        gateway,
                        dns1,
                        dns2,
                    } => network_profile::eth::Ip::Static(network_profile::Static {
                        address,
                        netmask,
                        gateway,
                        dns1,
                        dns2,
                    }),
                }),
            }),
            wlan: Some(network_profile::Wlan {
                name: form.wlan.name.clone(),
                wifi: form.wlan.wifi.map(|wifi| match wifi {
                    Wifi::Open { ssid } => {
                        network_profile::wlan::Wifi::Open(network_profile::Open { ssid })
                    }
                    Wifi::Psk { ssid, password } => {
                        network_profile::wlan::Wifi::Psk(network_profile::Psk { ssid, password })
                    }
                    Wifi::Eap {
                        ssid,
                        identity,
                        password,
                    } => network_profile::wlan::Wifi::Eap(network_profile::Eap {
                        ssid,
                        identity,
                        password,
                    }),
                }),
                ip: Some(match form.wlan.ip {
                    Ip::Dhcp => network_profile::wlan::Ip::Dhcp(true),
                    Ip::Static {
                        address,
                        netmask,
                        gateway,
                        dns1,
                        dns2,
                    } => network_profile::wlan::Ip::Static(network_profile::Static {
                        address,
                        netmask,
                        gateway,
                        dns1,
                        dns2,
                    }),
                }),
            }),
        }))
    }

    async fn set_network(&self, req: Request<NetworkProfile>) -> GrpcResult<()> {
        current_pi_user!(self, &req);
        // TODO
        Ok(Response::new(()))
    }

    async fn get_ntp(&self, req: Request<()>) -> GrpcResult<NtpProfile> {
        current_pi_user!(self, &req);
        let db = try_grpc!(self.db.get())?;
        let db = db.deref();
        let aes = self.aes.deref();
        let it: Ntp = SettingDao::get(db, aes, Ntp::KEY).unwrap_or_default();

        Ok(Response::new(NtpProfile {
            enable: it.enable,
            timezone: it.timezone.name().to_string(),
            servers: it.servers,
            heartbeat: it.heartbeat as u64,
        }))
    }

    async fn set_ntp(&self, req: Request<NtpProfile>) -> GrpcResult<()> {
        current_pi_user!(self, &req);
        let db = try_grpc!(self.db.get())?;
        let db = db.deref();
        let req = req.into_inner();

        let form = Ntp {
            enable: req.enable,
            timezone: req.timezone.parse().map_err(Status::invalid_argument)?,
            servers: req.servers,
            heartbeat: req.heartbeat as usize,
        };

        try_grpc!(form.save(db, &self.aes))?;
        Ok(Response::new(()))
    }

    async fn get_vpn(&self, req: Request<()>) -> GrpcResult<VpnProfile> {
        current_pi_user!(self, &req);
        let db = try_grpc!(self.db.get())?;
        let db = db.deref();
        let aes = self.aes.deref();
        let it: Vpn = SettingDao::get(db, aes, Vpn::KEY).unwrap_or_default();

        Ok(Response::new(VpnProfile {
            enable: it.enable,
            body: it.body,
        }))
    }

    async fn set_vpn(&self, req: Request<VpnProfile>) -> GrpcResult<()> {
        current_pi_user!(self, &req);
        let db = try_grpc!(self.db.get())?;
        let db = db.deref();
        let req = req.into_inner();

        let form = Vpn {
            enable: req.enable,
            body: req.body,
        };

        let aes = self.aes.deref();
        try_grpc!(form.save(db, aes))?;
        Ok(Response::new(()))
    }
}

#[derive(Validate)]
pub struct Ping {
    #[validate(length(min = 1))]
    pub host: String,
}

impl Ping {
    pub fn execute(&self) -> Result<String> {
        self.validate()?;
        let out = Command::new("ping")
            .arg("-W")
            .arg("2")
            .arg("-c")
            .arg("4")
            .arg(&self.host)
            .output()?;
        debug!("{:?}", out);
        Ok(String::from_utf8(out.stdout)?)
    }
}

#[derive(Validate)]
pub struct Dns {
    #[validate(length(min = 1))]
    pub host: String,
    #[validate(length(min = 1))]
    pub server: Option<String>,
}

impl Dns {
    pub fn execute(&self) -> Result<String> {
        self.validate()?;
        let out = match self.server {
            Some(ref it) => Command::new("dig")
                .arg(&format!("@{}", it))
                .arg(&self.host)
                .output(),
            None => Command::new("dig").arg(&self.host).output(),
        }?;
        debug!("{:?}", out);
        Ok(String::from_utf8(out.stdout)?)
    }
}

#[derive(Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Ntp {
    pub timezone: Tz,
    #[validate(length(min = 1))]
    pub servers: Vec<String>,
    #[validate(range(min = 5))]
    pub heartbeat: usize,
    pub enable: bool,
}

impl Default for Ntp {
    fn default() -> Self {
        Self {
            timezone: Tz::UTC,
            servers: vec!["0.us.pool.ntp.org".to_string()],
            heartbeat: 60 * 60 * 24,
            enable: false,
        }
    }
}

impl Ntp {
    pub fn timesyncd(&self) -> String {
        format!(
            r#"
[Time]
NTP={servers}
FallbackNTP=0.pool.ntp.org 1.pool.ntp.org 2.pool.ntp.org 3.pool.ntp.org
        "#,
            servers = self.servers.join(" ")
        )
    }
    pub fn crontab(&self) -> String {
        format!(
            r#"
#!/bin/bash
for i in {servers}
do
    ntpdate $i && break
done
        "#,
            servers = self.servers.join(" ")
        )
    }

    pub fn test(&self) -> Result<Vec<DateTime<Local>>> {
        let mut items = Vec::new();
        for it in self.servers.iter() {
            let now: DateTime<Local> = NtpResponse::fetch(it, None)?.into();
            items.push(now);
        }
        Ok(items)
    }

    pub fn ping(&self) -> Option<DateTime<Local>> {
        for it in self.servers.iter() {
            if let Ok(it) = NtpResponse::fetch(it, None) {
                return Some(it.into());
            }
        }
        None
    }

    pub const KEY: &'static str = "ntp.client";
    pub fn save(&self, db: &Db, aes: &Aes) -> Result<()> {
        self.validate()?;
        self.test()?;
        debug!("save ntp server {:?}", self);
        let file = Path::new(&Component::RootDir)
            .join("etc")
            .join("systemd")
            .join("timesyncd.conf");

        if self.enable {
            let mut fd = File::create(&file)?;
            write!(&mut fd, "{}", self.timesyncd())?;
        } else if file.exists() {
            remove_file(&file)?;
        }
        SettingDao::set(db, aes, Self::KEY, self, true)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Validate, Default)]
#[serde(rename_all = "camelCase")]
pub struct Vpn {
    pub enable: bool,
    #[validate(length(min = 1))]
    pub body: String,
}

impl Vpn {
    pub const KEY: &'static str = "openvpn.client";
    pub fn save(&self, db: &Db, aes: &Aes) -> Result<()> {
        self.validate()?;
        let file = Path::new(&Component::RootDir)
            .join("etc")
            .join("openvpn")
            .join("client.conf");
        if self.enable {
            info!("generate file {}", file.display());
            let mut fd = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .mode(0o600)
                .open(file)?;
            fd.write_all(self.body.as_bytes())?;
        } else if file.exists() {
            info!("delete file {}", file.display());
            remove_file(file)?;
        }
        SettingDao::set(db, aes, Self::KEY, self, true)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Wlan {
    pub name: String,
    pub wifi: Option<Wifi>,
    pub ip: Ip,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Eth {
    pub name: String,
    pub ip: Ip,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Network {
    pub eth: Eth,
    pub wlan: Wlan,
}

impl Default for Network {
    fn default() -> Self {
        Self {
            eth: Eth {
                name: Self::ETH.to_string(),
                ip: Ip::default(),
            },
            wlan: Wlan {
                name: Self::WLAN.to_string(),
                ip: Ip::default(),
                wifi: None,
            },
        }
    }
}

impl Network {
    pub const KEY: &'static str = "systemd.network";
    pub const ETH: &'static str = "eth0";
    pub const WLAN: &'static str = "wlan0";

    #[cfg(debug_assertions)]
    pub fn mac(&self) -> Result<MacAddress> {
        get_mac("wlp3s0")
    }
    #[cfg(not(debug_assertions))]
    pub fn mac(&self) -> Result<MacAddress> {
        get_mac(Self::ETH)
    }
    pub fn is_on(&self) -> bool {
        (is_on(&self.eth.name) && get_ip4(&self.eth.name).is_some())
            || (is_on(&self.wlan.name) && get_ip4(&self.wlan.name).is_some())
    }
    pub fn save(&self, vendor: &str, db: &Db, aes: &Aes) -> Result<()> {
        debug!("save network interfaces {:?}", self);
        {
            let metric = 50;
            match self.eth.ip {
                Ip::Static {
                    ref address,
                    ref netmask,
                    ref gateway,
                    ref dns1,
                    ref dns2,
                } => Static::new(
                    &self.eth.name,
                    metric,
                    address,
                    netmask,
                    gateway,
                    dns1,
                    dns2.as_deref(),
                )?
                .save(vendor)?,
                Ip::Dhcp => Dhcp {
                    name: self.eth.name.clone(),
                    metric,
                    options: vec![Dhcp::WWW],
                }
                .save(vendor)?,
            };
        }

        {
            let metric = 200;
            match self.wlan.wifi {
                Some(ref it) => {
                    it.save(&self.wlan.name)?;
                    Dhcp {
                        name: self.wlan.name.clone(),
                        metric,
                        options: vec![Dhcp::WWW],
                    }
                    .save(vendor)?;
                    Wpa.save(&self.wlan.name)?;
                }
                None => {
                    Wifi::remove(&self.wlan.name)?;
                }
            }
        }
        SettingDao::set(db, aes, Self::KEY, self, true)?;
        Ok(())
    }
}
