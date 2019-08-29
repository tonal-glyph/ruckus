use rosc;
use osc_address as oa;
use serde_bytes as sb;
use serde_osc as so;
use rosc::{OscPacket,OscMessage,OscType};
use rosc::encoder;
pub fn main() {
    let packet = OscPacket::Message(OscMessage{
            addr: "/greet/me".to_string(),
            args: Some(vec![OscType::String("hi!".to_string())])
        }
    );
    assert!(encoder::encode(&packet).is_ok())
}
