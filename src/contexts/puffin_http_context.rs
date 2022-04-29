use puffin_http::{Server, Client};

use crate::*;

pub struct PuffinHttpContext {
    pub server: Server,
    _client: Client,
}

impl PuffinHttpContext {
    pub fn new(server_address: &str) -> Result<Self, anyhow::Error> {
        let server_result = Server::new(server_address);
        if server_result.is_ok() {
            let client = Client::new("127.0.0.1:8585".to_owned());
            Ok(Self {
                server: server_result.unwrap(),
                _client: client
            })
        } else {
            Err(server_result.err().unwrap())
        }
    }
}

impl Subcontext for PuffinHttpContext {}
