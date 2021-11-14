#![allow(dead_code)]

use crate::core::sonetserver::SonetServer;
use crate::core::sonetclient::SonetClient;
use crate::api::components::server::Server;
use crate::api::components::client::Client;

pub struct Sonet;

impl Sonet {
    fn create_client() -> SonetClient {
        SonetClient::new()
    }
    
    fn create_server(port: &i16) -> SonetServer {
        SonetServer::new(port)
    }
}