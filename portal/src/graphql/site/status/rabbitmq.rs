use juniper::GraphQLObject;

use super::super::super::super::queue::rabbitmq::Client;
#[derive(Debug, Clone, Default, GraphQLObject)]
#[graphql(name = "RabbitMQStatus")]
pub struct Item {
    pub connected: bool,
    pub frame_max: i32,
    pub channel_max: i32,
    pub heartbeat: i32,
}

impl Item {
    pub fn new(client: &Client) -> Self {
        let config = client.configuration();
        let status = client.status();
        Self {
            connected: status.connected(),
            channel_max: config.channel_max() as i32,
            frame_max: config.frame_max() as i32,
            heartbeat: config.heartbeat() as i32,
        }
    }
}
