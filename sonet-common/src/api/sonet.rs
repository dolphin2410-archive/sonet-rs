#![allow(dead_code)]

use crate::core::sonetserver::SonetServer;
use crate::core::sonetclient::SonetClient;
use crate::api::components::server::Server;
use crate::api::components::client::Client;
use crate::api::util::java_types::Int;

pub struct Sonet;

impl Sonet {
    pub fn create_client() -> SonetClient {
        SonetClient::new()
    }
    
    pub fn create_server(port: Int) -> SonetServer<> {
        SonetServer::new(port)
    }
}