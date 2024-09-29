use casbin::prelude::*;
use diesel_adapter::DieselAdapter;
use petunia::Result;

pub async fn new<U: Into<String>>(url: U, pool_size: u32) -> Result<Enforcer> {
    let model =
        DefaultModel::from_file(include_str!("rbac_with_resource_roles_model.conf")).await?;
    let adapter = DieselAdapter::new(url, pool_size)?;
    let enforcer = Enforcer::new(model, adapter).await?;
    Ok(enforcer)
}
