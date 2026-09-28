use std::fmt;
use std::path::{Path, PathBuf};

use chrono::{DateTime, NaiveDateTime};
use git2::{
    Commit, Cred, FetchOptions, RemoteCallbacks, Repository, Signature, Sort,
    build::{CheckoutBuilder, RepoBuilder},
};
use hyper::StatusCode;
use nix::unistd::{Uid, User};
use serde::{Deserialize, Serialize};

use super::{HttpError, Result};

pub struct Git {
    repository: Repository,
}

impl Git {
    pub fn commit_logs(&self) -> Result<Vec<CommitLog>> {
        let mut revwalk = self.repository.revwalk()?;
        revwalk.set_sorting(Sort::TIME)?;
        revwalk.push_head()?;

        let mut items = Vec::new();
        for oid in revwalk {
            let oid = oid?;
            let commit = self.repository.find_commit(oid)?;

            items.push(CommitLog::new(&commit)?);
        }
        Ok(items)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommitLog {
    pub id: String,
    pub short_id: String,
    pub message: String,
    pub created_at: NaiveDateTime,
    pub author: Author,
}

impl CommitLog {
    pub fn new(commit: &Commit) -> Result<Self> {
        let it = Self {
            id: commit.id().to_string(),
            author: Author::new(&commit.author())?,
            message: commit.message()?.trim().to_string(),
            created_at: {
                let t = commit.time().seconds();
                DateTime::from_timestamp(t, 0)
                    .ok_or_else(|| {
                        Box::new(HttpError(
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Some(format!("invalid timestamp {t}")),
                        ))
                    })?
                    .naive_utc()
            },
            short_id: {
                let obj = commit.as_object();
                let buf = obj.short_id()?;
                let it = buf.as_str()?;
                it.to_string()
            },
        };
        Ok(it)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub name: String,
    pub email: String,
}
impl fmt::Display for Author {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}<{}>", self.name, self.email)
    }
}
impl Author {
    pub fn new(signature: &Signature) -> Result<Self> {
        let it = Self {
            name: signature.name()?.to_string(),
            email: signature.email()?.to_string(),
        };
        Ok(it)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Host {
    Http {
        url: String,
        account: Option<(String, String)>,
    },
    Ssh {
        host: String,
        port: Option<u16>,
        username: Option<String>,
        private_key_file: Option<PathBuf>,
    },
}

impl fmt::Display for Host {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Http { url, account: _ } => write!(f, "{}", url),
            Self::Ssh {
                host,
                port,
                username,
                private_key_file: _,
            } => write!(
                f,
                "{}@{}:{}",
                username.as_deref().unwrap_or(Self::DEFAULT_SSH_USER),
                host,
                port.unwrap_or(Self::DEFAULT_SSH_PORT)
            ),
        }
    }
}

impl Host {
    const DEFAULT_SSH_USER: &str = "git";
    const DEFAULT_SSH_PORT: u16 = 22;
    pub fn open<P: AsRef<Path>>(&self, target_dir: P, branch: &str) -> Result<Git> {
        let target_dir = target_dir.as_ref();
        if target_dir.exists() {
            let repo = Repository::open(target_dir)?;
            let mut remote = repo.find_remote("origin")?;
            let mut fetch_options = self.fetch_options()?;
            remote.fetch(&[branch], Some(&mut fetch_options), None)?;
            let fetch_head = repo.find_reference("FETCH_HEAD")?;
            let fetch_commit = fetch_head.peel_to_commit()?;
            let merge_target = repo.reference_to_annotated_commit(&fetch_head)?;
            let (merge_analysis, _) = repo.merge_analysis(&[&merge_target])?;

            if merge_analysis.is_fast_forward() {
                let ref_name = format!("refs/heads/{}", branch);
                let mut local_ref = repo.find_reference(&ref_name)?;

                local_ref.set_target(fetch_commit.id(), "Fast-Forward Pull")?;

                repo.set_head(&ref_name)?;
                repo.checkout_head(Some(CheckoutBuilder::new().force()))?;
                log::info!("fast-forwarded to {}", fetch_commit.id());
            } else if merge_analysis.is_up_to_date() {
                log::info!("Repository is already up to date.");
            } else if merge_analysis.is_normal() {
                log::warn!("normal merge required (Not handled in this simple FF example).");
            }
        } else {
            let mut builder = RepoBuilder::new();
            builder.branch(branch).fetch_options(self.fetch_options()?);
            log::info!("clone to {}", target_dir.display());
            builder.clone(&self.url(), target_dir)?;
        }

        Ok(Git {
            repository: Repository::open(target_dir)?,
        })
    }

    fn url(&self) -> String {
        match self {
            Self::Http { url, account: _ } => url.clone(),
            Self::Ssh {
                host,
                port,
                username,
                private_key_file: _,
            } => format!(
                "ssh://{}@{}:{}",
                username.as_deref().unwrap_or(Self::DEFAULT_SSH_USER),
                host,
                port.unwrap_or(Self::DEFAULT_SSH_PORT)
            ),
        }
    }

    fn fetch_options(&self) -> Result<FetchOptions<'_>> {
        let mut callbacks = RemoteCallbacks::new();
        if let Self::Http {
            url: _,
            account: Some((username, password)),
        } = self
        {
            callbacks.credentials(|_url, _username_from_url, _allowed_types| {
                Cred::userpass_plaintext(username, password)
            });
        } else if let Self::Ssh {
            host: _,
            port: _,
            username,
            private_key_file,
        } = self
        {
            let user = {
                let uid = Uid::current();
                User::from_uid(uid)?
                    .ok_or_else(|| Box::new(HttpError(StatusCode::INTERNAL_SERVER_ERROR, None)))?
            };

            callbacks.credentials(move |_url, _username_from_url, _allowed_types| {
                let key = user.dir.join(".ssh").join("id_ed25519.pub");
                Cred::ssh_key(
                    username.as_deref().unwrap_or(Self::DEFAULT_SSH_USER),
                    None,
                    match private_key_file {
                        Some(it) => it.as_path(),
                        None => key.as_path(),
                    },
                    None,
                )
            });
        }

        let mut options = FetchOptions::new();
        options.remote_callbacks(callbacks);
        Ok(options)
    }
}
