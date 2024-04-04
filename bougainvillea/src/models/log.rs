use std::collections::HashSet;
use std::ffi::OsStr;
use std::path::Path;
use std::process::Command;

use chrono::NaiveDate;
use hyper::StatusCode;
use log::debug;
use palm::{HttpError, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Item {
    #[serde(rename = "PRIORITY")]
    pub priority: u8,
    #[serde(rename = "_HOSTNAME")]
    pub hostname: String,
    #[serde(rename = "_TRANSPORT")]
    pub transport: String,
    #[serde(rename = "__REALTIME_TIMESTAMP")]
    pub timestamp: i64,
    #[serde(rename = "MESSAGE")]
    pub message: String,
    #[serde(rename = "_SYSTEMD_UNIT")]
    pub systemd_unit: Option<String>,
}

macro_rules! shell_command_output {
    ($x:expr) => {{
        debug!("{}", $x.status);
        if $x.status.success() {
            Ok(String::from_utf8($x.stdout)?)
        } else {
            Err(Box::new(HttpError(
                StatusCode::BAD_REQUEST,
                Some(String::from_utf8($x.stderr)?),
            )))
        }
    }};
}

impl Item {
    pub const SHOW: &'static str = "show";

    pub fn load<P, U>(
        dir: Option<P>,
        unit: Option<U>,
        since: NaiveDate,
        until: NaiveDate,
    ) -> Result<Vec<Self>>
    where
        P: AsRef<Path>,
        U: AsRef<OsStr>,
    {
        let output = {
            let mut cmd = Self::command(dir);
            cmd.arg("--since")
                .arg(since.to_string())
                .arg("--until")
                .arg(until.to_string())
                .arg("--output")
                .arg("json")
                .arg("--reverse")
                .arg("--utc");

            match unit {
                Some(unit) => cmd.arg("--unit").arg(unit),
                None => cmd.arg("--all"),
            };
            cmd.output()?
        };

        let output = shell_command_output!(output)?;
        let mut items = Vec::new();
        for line in output.lines() {
            let it = serde_json::from_str(line)?;
            items.push(it);
        }
        Ok(items)
    }

    pub fn systemd_units<P: AsRef<Path>>(dir: Option<P>) -> Result<HashSet<String>> {
        Self::fields(dir, "_SYSTEMD_UNIT")
    }
    pub fn hostnames<P: AsRef<Path>>(dir: Option<P>) -> Result<HashSet<String>> {
        Self::fields(dir, "_HOSTNAME")
    }
    pub fn fields<P: AsRef<Path>>(dir: Option<P>, column: &str) -> Result<HashSet<String>> {
        let output = Self::command(dir).arg("--field").arg(column).output()?;
        let output = shell_command_output!(output)?;

        let items = output
            .lines()
            .map(|x| x.trim().to_string())
            .filter(|x| !x.is_empty())
            .collect();
        Ok(items)
    }

    fn command<P: AsRef<Path>>(dir: Option<P>) -> Command {
        let mut it = Command::new("/usr/bin/journalctl");
        if let Some(dir) = dir {
            let dir = dir.as_ref();
            it.arg("--directory").arg(format!("{}", dir.display()));
        }
        it
    }
}
