extern crate pnet; 
use pnet::datalink::{self, NetworkInterface};
use pnet::datalink::Channel::Ethernet;
use pnet::packet::{Packet, MutablePacket};

pub fn setup_interface(inter_name: &str) 
{
    let interface_names_match = |iface: &NetworkInterface | iface.name == inter_name; 
    let interfaces = datalink::interfaces(); 
    let interface = interfaces.into_iter()
                              .filter(interface_names_match)
                              .next()
                              .unwrap() 
    let (mut sender,mut reciever) = match datalink::channel(interface, Default::default()){
        Ok(Ethernet(tx,rx)) => (tx,rx),
        Ok(_) => panic!("unhandled channel type"),
        Err(e) => panic!("error occured {}",e),
    };


}
