use proto::zmessage::message_client::MessageClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = MessageClient::connect("http://127.0.0.1:10020").await?;
    let msg = proto::zmessage::ZMessage {
        version: 0,
        r#type: 0,
        public_key: vec![],
        data: Vec::from("Are you ok?".to_string()),
        signature: vec![],
    };
    let response = client.send_z_message(msg).await?;
    let d = response.into_inner().data;
    if let Ok(s) = std::str::from_utf8(&d) {
        println!("Response: {}", s);
    }
    Ok(())
}
