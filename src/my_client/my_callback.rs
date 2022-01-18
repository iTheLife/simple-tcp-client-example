use async_trait::async_trait;
use my_tcp_sockets::{ConnectionEvent, SocketEventCallback};

use super::MySerializer;

pub struct MyCallback {}

impl MyCallback {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl SocketEventCallback<String, MySerializer> for MyCallback {
    async fn handle(&self, connection_event: ConnectionEvent<String, MySerializer>) {
        match connection_event {
            ConnectionEvent::Connected(_) => {
                println!("Connected");
            }
            ConnectionEvent::Disconnected(_) => {
                println!("Disconnected");
            }
            ConnectionEvent::Payload {
                connection: _,
                payload,
            } => {
                println!("payload: {}", payload);
            }
        }
    }
}
