// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }

extern crate serde_derive;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkOptions {
    #[serde(rename = "container_id")]
    pub container_id: Option<String>,

    #[serde(rename = "container_name")]
    pub container_name: Option<String>,

    #[serde(rename = "networks")]
    pub networks: Option<HashMap<String, PerNetworkOptions>>,

    #[serde(rename = "port_mappings")]
    pub port_mappings: Option<Vec<PortMapping>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PerNetworkOptions {
    #[serde(rename = "aliases")]
    pub aliases: Option<Vec<String>>,

    #[serde(rename = "interface_name")]
    pub interface_name: Option<String>,

    #[serde(rename = "static_ips")]
    pub static_ips: Option<Vec<String>>,

    #[serde(rename = "static_mac")]
    pub static_mac: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PortMapping {
    #[serde(rename = "container_port")]
    pub container_port: i64,

    #[serde(rename = "host_ip")]
    pub host_ip: Option<String>,

    #[serde(rename = "host_port")]
    pub host_port: Option<i64>,

    #[serde(rename = "protocol")]
    pub protocol: Option<String>,

    #[serde(rename = "range")]
    pub range: Option<i64>,
}
