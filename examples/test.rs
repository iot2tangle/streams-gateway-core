use gateway_core::gateway::publisher::Channel;
use iota_streams::app::transport::tangle::client::SendTrytesOptions;

#[derive(serde::Serialize)]
struct Data {
    hello: String,
    world: String,
}

///
/// Opens a Stream channel and sends some data to it
///
fn main() {
    let mut send_opt = SendTrytesOptions::default();
    send_opt.local_pow = false;

    let mut channel = Channel::new("https://nodes.iota.cafe:443".to_string(), send_opt, None);

    let address = channel.open().unwrap();
    println!("Channel Address: {}", address);

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
