use eui48::MacAddress;
use serde::{Deserialize, Serialize};

use super::super::super::super::{
    crypto::Aes,
    jwt::Jwt,
    orm::sqlite::Connection as Db,
    request::Token,
    sys::network::{
        ip4 as get_ip4, is_on, mac as get_mac,
        systemd::{Dhcp, Network, Static, Wifi, Wpa},
    },
    Result,
};
use super::super::models::settings::Dao as SettingDao;
use super::user::CurrentUser;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Wlan {
    pub name: String,
    pub wifi: Option<Wifi>,
    pub network: Network,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Eth {
    pub name: String,
    pub network: Network,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Form {
    pub eth: Eth,
    pub wlan: Wlan,
}

impl Default for Form {
    fn default() -> Self {
        Self {
            eth: Eth {
                name: "eth0".to_string(),
                network: Network::default(),
            },
            wlan: Wlan {
                name: "wlan0".to_string(),
                network: Network::default(),
                wifi: None,
            },
        }
    }
}
impl Form {
    pub const KEY: &'static str = "systemd.network";
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
    pub fn new(ss: &Token, db: &Db, jwt: &Jwt, aes: &Aes) -> Result<Self> {
        ss.current_user(db, jwt, aes)?;
        let it: Self = SettingDao::get(db, aes, Self::KEY).unwrap_or_default();
        Ok(it)
    }
    pub fn save(&self, vendor: &str, ss: &Token, db: &Db, jwt: &Jwt, aes: &Aes) -> Result<()> {
        ss.current_user(db, jwt, aes)?;
        SettingDao::set(db, aes, Self::KEY, self, true)?;
        debug!("save network interfaces {:?}", self);
        {
            let metric = 50;
            match self.eth.network {
                Network::Static {
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
                    Some(dns2),
                )?
                .save(vendor)?,
                Network::Dhcp => Dhcp {
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
        Ok(())
    }
}
