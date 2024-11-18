use juniper::GraphQLObject;
use lapin::Connection;
use petunia::Result;

#[derive(GraphQLObject)]
#[graphql(name = "RabbitMQStatus")]
pub struct Status {
    pub username: String,
    pub virtual_host: String,
}

impl Status {
    pub async fn new(con: &Connection) -> Result<Self> {
        let cs = con.status();
        let it = Self {
            username: cs.username(),
            virtual_host: cs.vhost(),
        };
        Ok(it)
    }
}
