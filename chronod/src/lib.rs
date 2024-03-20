use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

use api::*;
use proto::zmessage::ZMessage;

pub mod clock;

pub struct ConsensusTest {
    //  pub buf: Vec<u8>,
    pub receive: Receiver<ZMessage>,
    pub send: Sender<ZMessage>,
}

pub fn init() -> ConsensusTest {
    let (sender, receiver) = mpsc::channel();
    ConsensusTest { receive: receiver, send: sender }
}
