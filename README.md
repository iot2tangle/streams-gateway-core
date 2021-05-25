# Streams-Gateway-Core

## Introdution
This is the Core Repository for the iot2tangle Gateways. It provides the base API to open Channels and publish signed data to the Tangle.  
  
To learn more about IOTA-Streams clicke [here](https://www.iota.org/solutions/streams)  
  
To look at example implementations we have two Gateways, for [HTTP](https://github.com/iot2tangle/Streams-wifi-gateway) and [MQTT](https://github.com/iot2tangle/streams-mqtt-gateway), already deployed with the Streams-Gateway-Core.
  
## Usage
To interact with Library import it as a dependency by adding it to the `Cargo.toml` file:  
`gateway_core = { git = "https://github.com/iot2tangle/streams-gateway-core", branch="master"}`
  
You can then import the library into your project with:  
`extern crate gateway_core;`  

## API 
To <b>Create a new channel</b>  use:  
`Channel::new(node: String, send_opt: SendTrytesOptions, seed_option: Option<String>);`  
* node is: the url of an IOTA Node   
* send_opt:  are options used by the IOTA Client when sending Transaction  
* seed_option: Can be None to generate a new Seed or Some(seed) to use an existing seed  

 <br>
  
   
To <b>Open the channel</b>  and get its address:    
`let address: String = channel.open().unwrap();`  
This will open the Channel by generating the channel address and publishing the signature keys  
This address will be needed to read the data from the Tangle  
  
   <br>
   
To <b>Send signed data</b> over the Tangle by writing to the channel:  
`write_signed<T>(&mut self, data: T) -> Result<String>`  
* The type T needs to have the `serde::Serialize` trait  
  
If the transaction is succesfully sent the Identifier of the transaction will be returned.  

## Example
To run the provided example:  
`cargo run --example test`  
This will send a "Hello World" message to the Tangle!

  
## Note
The Underlying libraries are not compatible for 32Bit Processors and Operating systems.  
