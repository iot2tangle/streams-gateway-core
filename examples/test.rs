use gateway_core::gateway::publisher::Channel;

#[derive(serde::Serialize)]
struct Data {
    hello: String,
    world: String,
}

///
/// Opens a Stream channel and sends some data to it
///
fn main() {
    let mut channel = Channel::new("https://nodes.iota.cafe:443".to_string(), 14, false, None);

    let (address, msg_id) = channel.open().unwrap();
    println!("Channel Address: {}", format!("{}:{}", address, msg_id));

    let data = Data {
        hello: String::from("Hello"),
        world: String::from("World!"),
    };

    match channel.write_signed(data) {
        Ok(_) => {
            println!("All good");
        }
        Err(_e) => {
            println!("This isn't working....");
        }
    };
}
