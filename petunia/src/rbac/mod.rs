pub mod watcher;

pub mod v1 {
    tonic::include_proto!("palm.rbac.v1");
}

use std::any::type_name;
use std::fmt;

use data_encoding::BASE64_NOPAD;
use prost::Message;

impl v1::policy_users_response::Item {
    pub fn by_id(id: i32) -> Self {
        Self {
            id: Some(v1::policy_users_response::item::Id::I(id)),
        }
    }
    pub fn by_code(code: String) -> Self {
        Self {
            id: Some(v1::policy_users_response::item::Id::S(code)),
        }
    }
}

impl fmt::Display for v1::policy_users_response::Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = {
            let mut buf = Vec::new();
            self.encode(&mut buf).map_err(|e| {
                log::error!("{}", e);
                fmt::Error
            })?;
            BASE64_NOPAD.encode(&buf)
        };
        write!(f, "{}", s)
    }
}

impl v1::policy_roles_response::Item {
    pub fn root() -> Self {
        Self {
            by: Some(v1::policy_roles_response::item::By::Root(
                v1::policy_roles_response::item::Root {},
            )),
        }
    }
    pub fn administrator() -> Self {
        Self {
            by: Some(v1::policy_roles_response::item::By::Administrator(
                v1::policy_roles_response::item::Administrator {},
            )),
        }
    }
    pub fn by_code(code: String) -> Self {
        Self {
            by: Some(v1::policy_roles_response::item::By::Code(code)),
        }
    }
}

impl fmt::Display for v1::policy_roles_response::Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = {
            let mut buf = Vec::new();
            self.encode(&mut buf).map_err(|e| {
                log::error!("{}", e);
                fmt::Error
            })?;
            BASE64_NOPAD.encode(&buf)
        };
        write!(f, "{}", s)
    }
}

impl v1::policy_permissions_response::item::Operation {
    pub fn read() -> Self {
        Self {
            by: Some(v1::policy_permissions_response::item::operation::By::Read(
                v1::policy_permissions_response::item::operation::Read {},
            )),
        }
    }
    pub fn write() -> Self {
        Self {
            by: Some(v1::policy_permissions_response::item::operation::By::Write(
                v1::policy_permissions_response::item::operation::Write {},
            )),
        }
    }
    pub fn append() -> Self {
        Self {
            by: Some(
                v1::policy_permissions_response::item::operation::By::Append(
                    v1::policy_permissions_response::item::operation::Append {},
                ),
            ),
        }
    }
    pub fn execute() -> Self {
        Self {
            by: Some(
                v1::policy_permissions_response::item::operation::By::Execute(
                    v1::policy_permissions_response::item::operation::Execute {},
                ),
            ),
        }
    }
    pub fn credit() -> Self {
        Self {
            by: Some(
                v1::policy_permissions_response::item::operation::By::Credit(
                    v1::policy_permissions_response::item::operation::Credit {},
                ),
            ),
        }
    }
    pub fn debit() -> Self {
        Self {
            by: Some(v1::policy_permissions_response::item::operation::By::Debit(
                v1::policy_permissions_response::item::operation::Debit {},
            )),
        }
    }
    pub fn inquiry() -> Self {
        Self {
            by: Some(
                v1::policy_permissions_response::item::operation::By::Inquiry(
                    v1::policy_permissions_response::item::operation::Inquiry {},
                ),
            ),
        }
    }
}

impl fmt::Display for v1::policy_permissions_response::item::Operation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = {
            let mut buf = Vec::new();
            self.encode(&mut buf).map_err(|e| {
                log::error!("{}", e);
                fmt::Error
            })?;
            BASE64_NOPAD.encode(&buf)
        };
        write!(f, "{}", s)
    }
}

impl v1::policy_permissions_response::item::Resource {
    pub fn new<T>() -> Self {
        Self {
            r#type: type_name::<T>().to_string(),
            id: None,
        }
    }
    pub fn by_id<T>(id: i32) -> Self {
        Self {
            r#type: type_name::<T>().to_string(),
            id: Some(v1::policy_permissions_response::item::resource::Id {
                by: Some(v1::policy_permissions_response::item::resource::id::By::I(
                    id,
                )),
            }),
        }
    }
    pub fn by_code<T>(code: String) -> Self {
        Self {
            r#type: type_name::<T>().to_string(),
            id: Some(v1::policy_permissions_response::item::resource::Id {
                by: Some(v1::policy_permissions_response::item::resource::id::By::S(
                    code,
                )),
            }),
        }
    }
}

impl fmt::Display for v1::policy_permissions_response::item::Resource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = {
            let mut buf = Vec::new();
            self.encode(&mut buf).map_err(|e| {
                log::error!("{}", e);
                fmt::Error
            })?;
            BASE64_NOPAD.encode(&buf)
        };
        write!(f, "{}", s)
    }
}
