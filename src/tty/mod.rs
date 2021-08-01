pub mod g786;
pub mod modbus;

use std::fmt;
use std::io::{prelude::*, Error as IoError, ErrorKind as IoErrorKind};
use std::num::ParseIntError;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::result::Result as StdResult;
use std::str::FromStr;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use chrono::Utc;
use clap::Clap;
use hyper::StatusCode;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serialport::{available_ports, SerialPortBuilder, TTYPort};

use super::{queue::zero::Queue, HttpError, Result};

lazy_static! {
    static ref UUID: Mutex<u16> = Mutex::new(u16::MIN);
}

#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct Uuid(u16);

impl Default for Uuid {
    fn default() -> Self {
        loop {
            if let Ok(mut it) = UUID.lock() {
                *it += 1;
                if *it == u16::MAX {
                    *it = 1;
                }
                return Self(*it);
            }
        }
    }
}

impl fmt::Display for Uuid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:04X}", self.0)
    }
}

impl FromStr for Uuid {
    type Err = ParseIntError;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        let v = u16::from_str_radix(s, 16)?;
        Ok(Self(v))
    }
}

impl Uuid {
    pub fn null() -> Self {
        Self(u16::MAX)
    }
    pub fn zero() -> Self {
        Self(u16::MIN)
    }
}

#[derive(Clap)]
pub struct Tty {
    #[clap(short, long)]
    pub json_port: u16,
    #[clap(short, long)]
    pub raw_port: u16,
    #[clap(short, long)]
    pub name: PathBuf,
}

pub trait Decoder: Send + Sync {
    type Item: DeserializeOwned + Serialize + fmt::Debug + fmt::Display + Send + Sync;
    type Error: fmt::Debug + Send + Sync;
    fn decode(line: &[u8]) -> StdResult<Option<(Self::Item, usize)>, Self::Error>;
}

pub struct SerialPort {
    tty: Mutex<TTYPort>,
}

impl SerialPort {
    pub const ORAGNTE_PI_UART1: &'static str = "/dev/ttyS1";
    pub const ORAGNTE_PI_UART2: &'static str = "/dev/ttyS2";
    pub const USB0: &'static str = "/dev/ttyUSB0";
    pub const USB1: &'static str = "/dev/ttyUSB1";
    pub const RASPBERRY_PI_UART1: &'static str = "/dev/serial0";

    pub fn new<P: AsRef<Path>>(name: P) -> Result<Self> {
        let name = name.as_ref();
        Self::open(
            serialport::new(&name.display().to_string(), 9600).timeout(Duration::from_millis(10)),
        )
    }
    pub fn open(tty: SerialPortBuilder) -> Result<Self> {
        info!("open serial port {:?} in exclusive mode", tty);
        let mut tty = tty.open_native()?;
        tty.set_exclusive(true)?;
        Ok(Self {
            tty: Mutex::new(tty),
        })
    }

    pub fn ports() -> Result<Vec<String>> {
        let items = available_ports()?
            .into_iter()
            .map(|it| it.port_name)
            .collect::<Vec<_>>();
        Ok(items)
    }

    pub fn listen_out_decoder<T: Decoder>(&self, raw: u16, json: u16) -> Result<()> {
        self.listen_out(raw, json, &T::decode)
    }

    pub fn listen_out<T, E, F>(&self, raw: u16, json: u16, hnd: &F) -> Result<()>
    where
        T: Serialize + DeserializeOwned + fmt::Display + fmt::Debug + Send + Sync,
        E: fmt::Debug + Send + Sync,
        F: Fn(&[u8]) -> StdResult<Option<(T, usize)>, E> + Send + Sync,
    {
        if let Ok(port) = self.tty.lock() {
            info!("start response producer thread for {:?}", port);
        }
        let json = Queue::Tcp(None, json).pub_()?;
        let raw = Queue::Tcp(None, raw).pub_()?;
        let mut buf = Vec::new();
        loop {
            let mut line: Vec<u8> = vec![0; 1 << 8];
            if let Ok(mut port) = self.tty.lock() {
                match port.read(line.as_mut_slice()) {
                    Ok(len) => {
                        debug!("receive {} bytes", len);
                        buf.extend_from_slice(&line[..len]);
                        if let Ok(it) = std::str::from_utf8(&buf) {
                            debug!("buffer {}", it);
                        }
                        match hnd(&buf) {
                            Ok(payload) => {
                                if let Some((payload, end)) = payload {
                                    info!("match {}", payload);
                                    raw.send(&payload.to_string(), 0)?;
                                    json.send(&serde_json::to_vec(&payload)?, 0)?;
                                    buf = buf.split_off(end);
                                }
                            }
                            Err(e) => {
                                error!("{:?}", e);
                                buf.clear();
                            }
                        };
                        if buf.len() >= (1 << 16) {
                            warn!("clear buffer");
                            buf.clear();
                        }
                    }
                    Err(ref e) if e.kind() == IoErrorKind::TimedOut => {
                        warn!("read tty timeout");
                    }
                    Err(e) => {
                        return Err(e.into());
                    }
                }
            }
        }
    }

    pub fn listen_in(&self, name: &str) -> Result<()> {
        if let Ok(port) = self.tty.lock() {
            info!("start request consumer thread for {:?}", port);
        }
        let queue = Queue::Ipc(name.to_string()).pull()?;
        loop {
            let msg = queue.recv_msg(0)?;
            let msg = msg.deref();
            self.write(msg)?;
        }
    }

    fn write(&self, buf: &[u8]) -> Result<()> {
        if let Ok(buf) = std::str::from_utf8(buf) {
            info!("write {}", buf);
        }
        let now = Utc::now();
        for i in 1..u8::MAX {
            if let Ok(mut port) = self.tty.lock() {
                match port.write_all(buf) {
                    Ok(_) => {
                        debug!(
                            "try {} times for write tty {}, spend {}",
                            i + 1,
                            buf.len(),
                            (Utc::now() - now)
                        );
                        return Ok(());
                    }
                    Err(ref e) if e.kind() == IoErrorKind::TimedOut => {
                        warn!("write tty timeout");
                    }
                    Err(e) => {
                        error!("{:?}", e);
                    }
                };
            }
            thread::sleep(Duration::from_millis((i * 100).into()));
        }
        Err(Box::new(HttpError(
            StatusCode::BAD_REQUEST,
            Some("write tty failed.".to_string()),
        )))
    }
}

pub fn write<T: DeserializeOwned>(name: &str, buffer: &[u8], port: u16, ttl: i64) -> Result<T> {
    let raw = Queue::Ipc(name.to_string()).push()?;
    let js = Queue::Tcp(None, port).sub(None)?;
    raw.send(buffer, 0)?;
    let now = Utc::now();

    loop {
        let msg = js.recv_msg(0)?;
        let msg = msg.deref();
        if let Ok(it) = serde_json::from_slice(msg) {
            return Ok(it);
        }

        if (Utc::now() - now) >= chrono::Duration::seconds(ttl) {
            return Err(Box::new(IoError::new(
                IoErrorKind::TimedOut,
                "can't get the expect response",
            )));
        }
    }
}
