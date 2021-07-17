use std::io::{prelude::*, ErrorKind as IoErrorKind};
use std::ops::Deref;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use chrono::Utc;
use clap::Clap;
use serialport::{available_ports, SerialPortBuilder, TTYPort};

use super::{queue::zero::Queue, Result};

#[derive(Clap)]
pub struct Tty {
    #[clap(short, long)]
    pub port: u16,
    #[clap(short, long)]
    pub name: String,
}

impl Tty {
    pub fn open(&self) -> Result<TTYPort> {
        let tty = open(serialport::new(&self.name, 9600).timeout(Duration::from_millis(10)))?;
        Ok(tty)
    }
}

pub const ORAGNTE_PI_UART1: &str = "/dev/ttyS1";
pub const ORAGNTE_PI_UART2: &str = "/dev/ttyS2";
pub const USB0: &str = "/dev/ttyUSB0";
pub const USB1: &str = "/dev/ttyUSB1";
pub const RASPBERRY_PI_UART1: &str = "/dev/serial0";

pub fn ports() -> Result<Vec<String>> {
    let items = available_ports()?
        .into_iter()
        .map(|it| it.port_name)
        .collect::<Vec<_>>();
    Ok(items)
}

pub fn open(tty: SerialPortBuilder) -> Result<TTYPort> {
    info!("open serial port {:?} in exclusive mode", tty);
    let mut tty = tty.open_native()?;
    tty.set_exclusive(true)?;
    Ok(tty)
}

pub fn listen_out<F>(tty: &Mutex<TTYPort>, hnd: &F, port: u16) -> Result<()>
where
    F: Fn(&str) -> Option<(String, usize)> + Send + Sync,
{
    if let Ok(tty) = tty.lock() {
        info!("start response producer thread for {:?}", tty);
    }
    let queue = Queue::Tcp(None, port).pub_()?;
    let mut buf = String::new();
    loop {
        let mut line: Vec<u8> = vec![0; 1 << 8];
        if let Ok(mut port) = tty.lock() {
            match port.read(line.as_mut_slice()) {
                Ok(len) => {
                    debug!("receive {} bytes", len);
                    buf += std::str::from_utf8(&line[..len])?;
                    debug!("{}", buf);
                    if let Some((payload, end)) = hnd(&buf) {
                        info!("match {:?}", payload);
                        queue.send(&payload.as_bytes(), 0)?;
                        debug!("clear {:?}", buf[..end].to_string());
                        buf = buf[end..].to_string();
                    }
                    if buf.len() >= (1 << 16) {
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

pub fn listen_in(name: &str, tty: &Mutex<TTYPort>) -> Result<()> {
    if let Ok(tty) = tty.lock() {
        info!("start request consumer thread for {:?}", tty);
    }
    let queue = Queue::Ipc(name.to_string()).pull()?;
    loop {
        let msg = queue.recv_msg(0)?;
        let msg = msg.deref();
        write(tty, msg)?;
    }
}
fn write(port: &Mutex<TTYPort>, buf: &[u8]) -> Result<()> {
    let now = Utc::now();
    let mut i: usize = 1;
    loop {
        i += 1;
        if let Ok(mut port) = port.lock() {
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
        thread::sleep(Duration::from_micros(100));
    }
}
