pub mod models;

pub mod v1 {
    tonic::include_proto!("palm.rbac.v1");
}

use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};
use strum::{Display as EnumDisplay, EnumString};

#[derive(GraphQLObject)]
#[graphql(name = "Permission")]
pub struct Permission {
    pub resource: Resource,
    pub action: String,
}
impl Permission {
    pub fn new(it: &v1::Permission) -> Option<Self> {
        if let Some(ref rs) = it.resource {
            return Some(Self {
                resource: Resource::new(rs),
                action: it.action.clone(),
            });
        }
        None
    }
}

#[derive(GraphQLObject)]
#[graphql(name = "Resource")]
pub struct Resource {
    pub r#type: String,
    pub iid: Option<i32>,
    pub sid: Option<String>,
}

impl Resource {
    pub fn new(it: &v1::Resource) -> Self {
        Self {
            r#type: it.r#type.clone(),
            iid: if let Some(v1::resource::Id::I(i)) = it.id {
                Some(i)
            } else {
                None
            },
            sid: if let Some(v1::resource::Id::S(ref s)) = it.id {
                Some(s.clone())
            } else {
                None
            },
        }
    }
}

#[derive(EnumString, EnumDisplay, Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub enum Operation {
    Show,
    Edit,
    Delete,
}
