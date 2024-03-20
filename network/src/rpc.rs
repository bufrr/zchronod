use std::sync::{Arc, RwLock};

use log::{debug, error, info};
use log::kv::ToKey;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::Sender;
use tonic::{IntoRequest, Request, Response, Status, transport::Server};

use api::{CONTEXT, RT};
use proto::zmessage::message_server::{Message, MessageServer};
use proto::zmessage::ZMessage;
use storage::ZchronodDb;

#[derive(Clone)]
pub struct RpcServer {
    pub port: String,
}

impl RpcServer {
    pub fn new(self_address: &str) -> Self {
        RpcServer {
            port: self_address.to_string(),
        }
    }

    pub fn run(&self, zc: std::sync::mpsc::Sender<ZMessage>, db: Arc<RwLock<ZchronodDb>>) -> Result<(), Box<dyn std::error::Error>> {
        println!("[{}] start rpc listen on {}", module_path!(), self.port);
        let addr = self.port.parse()?;
        let server = Server::builder()
            .add_service(MessageServer::new(ZMessageService::new(zc, db)))
            .serve(addr);

        tokio::spawn(server);
        Ok(())
    }
}

pub struct ZMessageService {
    send_to_peers: std::sync::mpsc::Sender<ZMessage>,
    db: Arc<RwLock<ZchronodDb>>,
}

impl ZMessageService {
    pub fn new(send: std::sync::mpsc::Sender<ZMessage>, db: Arc<RwLock<ZchronodDb>>) -> Self {
        ZMessageService {
            send_to_peers: send,
            db,
        }
    }
}

#[tonic::async_trait]
impl Message for ZMessageService {
    async fn send_z_message(
        &self,
        _req: Request<ZMessage>,
    ) -> Result<Response<ZMessage>, Status> {
        self.send_to_peers.send(_req.into_inner()).unwrap();
        Ok(Response::new(ZMessage {
            version: 0,
            r#type: 0,
            public_key: vec![],
            data: Vec::from("Message received.".to_string()),
            signature: vec![],
        }))
    }
}



