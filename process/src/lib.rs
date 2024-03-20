use std::{fs, thread};
use std::cmp::Ordering;
use std::fs::File;
use std::io::Read;
use std::sync::{Arc, mpsc, Mutex, RwLock};
use std::sync::mpsc::{channel, Sender};

use async_std::io::Write;
use async_std::task as task1;
use bytes::Bytes;
use libp2p::{
    gossipsub,
    gossipsub::Message,
    mdns, Multiaddr,
    noise,
    PeerId, swarm::{NetworkBehaviour, SwarmEvent}, Swarm, tcp, yamux,
};
use libp2p::dns::ResolveErrorKind::Proto;
use log::{error, info};
use prost::Message as m1;
use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;
use tokio::task;

use api::{CONTEXT, NetworkInterface, Node};
use chronod::clock::{Clock, VlcMeta, VlcMsg, ZMessage};
use network::{GossipServer, RpcServer};
use proto::zchronod::Event;
use proto::zmessage::ZType;
use proto::zmessage::ZType::Rng;
use storage::ZchronodDb;

pub struct ZchronodServer {
    gossip_send: tokio::sync::mpsc::Sender<proto::zmessage::ZMessage>,
    node_address: String,
    // todo add config as node_config
    inner: Arc<RwLock<CoreZchronod>>,
    z_db: Arc<RwLock<ZchronodDb>>,
}

impl ZchronodServer {
    // assume rpc is a new from
    fn handle_rpc_msg(&self, x: proto::zmessage::ZMessage) {
        println!("receive from rpc {:?}", x);

        // construct publish event to gossip

        let mut inner = self.inner.write().unwrap();
        inner.count += 1;
        println!("current inner count is  {}", inner.count);
        println!("current Clock value is  {}， should+1", inner.clock.get_value());
        inner.clock.inc();


        // construct z_message
        let mut event_bytes = Vec::new();
        x.encode(&mut event_bytes).unwrap();

        let clock_msg = Clock {
            id: inner.clock.id.clone(),
            value: inner.clock.value,
            ancestors: vec![inner.clock.clone()],
        };
        let vlc_request_message = VlcMeta {
            clock_state: Some(clock_msg),
            event_meta: event_bytes,
        };
        let mut vlc_bytes = Vec::new();
        vlc_request_message.encode(&mut vlc_bytes).unwrap();

        let vlc_msg = VlcMsg {
            r#type: "request".to_string(),
            vlc_meta: vlc_bytes,
        };

        let mut z_mes_bytes = Vec::new();
        vlc_msg.encode(&mut z_mes_bytes).unwrap();

        let z_message = proto::zmessage::ZMessage {
            version: 0,
            r#type: Rng as i32,
            public_key: vec![],
            data: z_mes_bytes,
            signature: vec![],
        };

        println!("z-message construct ok, save to db, send to gossip");

        //self.distribute_event_msg_to_db(x);
        let rt = Runtime::new();
        rt.unwrap().block_on(self.gossip_send.send(
            z_message)).expect("failed to send to gossip");
    }

    fn distribute_event_msg_to_db(&self, e: proto::zmessage::ZMessage) {
        todo!()
    }

    fn construct_poll_event_key(&self, e: Event) -> String {
        // key is 3041_event-id_state
        let event_id: Vec<u8> = e.id.clone();
        println!("{:?}", event_id);
        let hex_string_back: String = hex::encode(e.id.clone());
        println!("{:?}", &hex_string_back);
        let constructed_string = format!("3041_{}_state", hex_string_back);
        // let result: String = event_id.iter()
        //     .map(|&num| num.to_string())
        //     .collect();
        constructed_string
    }

    pub fn handle_vlc_request(&self, vlc_meta: Vec<u8>) {
        let vlc_meta_instance = VlcMeta::decode(Bytes::from(vlc_meta)).unwrap();
        let clock_state = &vlc_meta_instance.clock_state.unwrap();
        println!("receive from gossip_clock_state_id [{}]", clock_state.id);
        match self.inner.write().unwrap().clock.partial_cmp(clock_state) {
            Some(Ordering::Greater) =>
                {
                    println!("dont need merge");
                    return;
                }
            _ => {
                println!("need merge");
            }
        }
        self.inner.write().unwrap().clock.merge(&vec![&clock_state]);
        //self.inner.write().unwrap().clock.inc();
        println!("current inner count is  {}", self.inner.read().unwrap().count);
        println!("current clock value is  {}, should+1", self.inner.read().unwrap().clock.get_value());

        //let e = Event::decode(Bytes::from(vlc_meta_instance.event_meta)).unwrap();
        //self.distribute_event_msg_to_db(e);;
    }
    pub fn handle_gossip_msg(&self, z_msg_bytes: Vec<u8>) {
        println!("handle gossip msg");
        let z_message = proto::zmessage::ZMessage::decode(Bytes::from(z_msg_bytes)).unwrap();
        match z_message.r#type {
            0 => {
                println!("handle vlc msg");
                let vlc_msg = VlcMsg::decode(Bytes::from(z_message.data)).unwrap();
                match vlc_msg.r#type.as_str() {
                    "request" => {
                        println!("handle vlc_request msg");
                        self.handle_vlc_request(vlc_msg.vlc_meta);
                    }
                    "sync" => {
                        //self.handle_vlc_sync(vlc_msg.vlc_meta);
                    }
                    _ => {
                        println!("unknown vlc type");
                    }
                }
            }
            _ => {
                println!("unknown z_message type");
            }
        }
    }
}

struct CoreZchronod {
    count: u8,
    clock: Clock,
}

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    id: String,
    peers: Vec<String>,
    rpc: RpcConfig,
    gossip: GossipConfig,
    db: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct RpcConfig {
    port: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct GossipConfig {
    port: String,
}

fn parse_config_file(file_path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(file_path)?;  // start with src/../..
    let config: Config = serde_yaml::from_str(&contents)?;
    Ok(config)
}

pub fn init_chrono_node(config: &str) {
    println!("{} init_chrono_node", config);
    let conf = parse_config_file(config).unwrap();
    println!("i am {}", &conf.id);
    //   let network = Network::init(&conf.peers, &conf.rpc.port, &conf.gossip.port);
    //  let cons = Consensus::init();
    let gossip = GossipServer::new(&conf.peers, &conf.gossip.port);
    let rpc = RpcServer::new(&conf.rpc.port);
    // CONTEXT = Some(api::Node::new(Box::new(network)));
    let db = Arc::new(RwLock::new(ZchronodDb::new(conf.db).unwrap()));

    run(gossip, db, rpc, conf.id);
    info!("[{}] zchronod service started",module_path!())
    // network::set().expect("TODO: panic message");
}

fn run(mut gossip: GossipServer<proto::zmessage::ZMessage>, db: Arc<RwLock<ZchronodDb>>, rpc: RpcServer, id: String) {
    let sender = gossip.send.clone();
    let sender_copy = gossip.send.clone();
    let consensus = Arc::new(chronod::init());
    let db_rpc_service = Arc::clone(&db);
    //let consensus_clone = Arc::clone(&consensus);
    rpc.run(consensus.send.clone(), db_rpc_service).expect("failed to run rpc");
    let (gossip_send, gossip_recv) = mpsc::channel::<(PeerId, Message)>();
    gossip.register_receive(gossip_send);

    let inner = Arc::new(RwLock::new(CoreZchronod { count: 0, clock: Clock::new(id) }));
    let inner_clone = Arc::clone(&inner);

    let db_c = Arc::clone(&db);
    thread::spawn(move || {
        loop {
            gossip_recv.iter().for_each(|(peer_id, message)| {
                println!("gossip receive here from {}", peer_id);
                let handle2 = ZchronodServer {
                    gossip_send: sender_copy.clone(),
                    node_address: "".to_string(),
                    inner: inner.clone(),
                    z_db: db.clone(),
                };
                thread::spawn(move || handle2.handle_gossip_msg(message.data));
            })
        }
    });

    thread::spawn(move || {
        let rt = Runtime::new().unwrap();
        rt.block_on(async { gossip.start().await })
    });


    loop {
        consensus.receive.iter().for_each(|x| {
            println!("rpc receive here which is {:?}", x);
            let handle1 = ZchronodServer {
                gossip_send: sender.clone(),
                node_address: "".to_string(),
                inner: inner_clone.clone(),
                z_db: db_c.clone(),
            };

            thread::spawn(move || handle1.handle_rpc_msg(x));
        });
    }
}
