use crate::network::types;
use crate::network::types::Subnet;
use std::net::IpAddr;

//  Teardown contains options for tearing down behind a container
#[derive(Clone, Debug)]
pub struct TeardownPortForward {
    pub config: PortForwardConfig,
    // remove network related information
    pub complete_teardown: bool,
}

//  SetupNetwork contains options for setting up a container
#[derive(Clone, Debug)]
pub struct SetupNetwork {
    // network object
    pub net: types::Network,
    // hash id for the network
    pub network_hash_name: String,
}

#[derive(Clone, Debug)]
pub struct TearDownNetwork {
    pub config: SetupNetwork,
    pub complete_teardown: bool,
}

#[derive(Clone, Debug)]
pub struct PortForwardConfig {
    //  network object
    pub net: types::Network,
    // id of container
    pub container_id: String,
    // port mappings
    pub port_mappings: Vec<types::PortMapping>,
    // name of network
    pub network_name: String,
    // hash id for the network
    pub network_hash_name: String,
    // ipv4 address of the container to bind to.
    // If multiple v4 addresses are present, use the first one for this.
    // At least one of container_ip_v6 and container_ip_v6 must be set. Both can
    // be set at the same time as well.
    pub container_ip_v4: Option<IpAddr>,
    // subnet associated with the IPv4 address.
    // Must be set if v4 address is set.
    pub subnet_v4: Option<Subnet>,
    // ipv6 address of the container.
    // If multiple v6 addresses are present, use the first one for this.
    // At least one of container_ip_v6 and container_ip_v6 must be set. Both can
    // be set at the same time as well.
    pub container_ip_v6: Option<IpAddr>,
    // subnet associated with the ipv6 address.
    // Must be set if the v6 address is set.
    pub subnet_v6: Option<Subnet>,
}

/// IPAMAddresses is used to pass ipam information around
pub struct IPAMAddresses {
    // ip addresses for netlink
    pub container_addresses: Vec<ipnet::IpNet>,
    pub gateway_addresses: Vec<ipnet::IpNet>,
    pub ipv6_enabled: bool,
    // result for podman
    pub net_addresses: Vec<types::NetAddress>,
    pub nameservers: Vec<IpAddr>,
}
