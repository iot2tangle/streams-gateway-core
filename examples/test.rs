use gateway_core::gateway::publisher::Channel;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct SensorData {
    pub iot2tangle: Vec<SensorType>,
    pub device: String,
    pub timestamp: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SensorType {
    pub sensor: String,
    pub data: Vec<Value>,
}

///
/// Opens a Stream channel and sends some data to it
///
fn main() {
    let mut channel = Channel::new(
        "https://chrysalis-nodes.iota.org:443".to_string(),
        false,
        None,
    );

    let (address, msg_id) = channel.open().unwrap();
    println!("Channel Address: {}", format!("{}:{}", address, msg_id));

    let data: SensorData = serde_json::from_str("{\"iot2tangle\":[{\"sensor\": \"Environmental\",\"data\":[{\"Pressure\":\"102033\"},{\"Temp\":\"26160\"},{\"Humidity\":\"33\"}]}],\"device\": \"DEVICE_ID_1\",\"timestamp\": \"1601653408\"}").unwrap();

    match channel.write_signed(data) {
        Ok(_) => {
            println!("All good");
        }
        Err(_e) => {
            println!("This isn't working....");
        }
    };
}
