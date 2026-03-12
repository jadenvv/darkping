extern crate pnet; 
use pnet::datalink::Channel::Ethernet;
use pnet::packet::{Packet, MutablePacket};
use tokio::sync::mpsc::{Sender, Receiver};
use pnet::datalink::{self, DataLinkSender, DataLinkReceiver, NetworkInterface};
use pnet::packet::ethernet::EthernetPacket;
use tokio::sync::{mpsc, Mutex, OnceCell}; 
use tokio::task;
use std::collections::HashMap; 
use std::thread::available_parrllelism;

pub(crate) type mpsc_comm<T> = (Sender<T>, Receiver<T>);

pub async fn suggested_interface() -> Option<NetworkInterface>
{
    let res= datalink::interfaces()
        .into_iter()
        .find(|iface: &NetworkInterface| !iface.is_loopback() && iface.is_up() && iface.mac.is_some() );
    res
}
pub async fn network_setup(iface: &NetworkInterface) -> (DataLinkSender, DatalinkReciever)
{
    let (mut sender, mut receiver)= match datalink::channel(&iface, Default::default()){
        Ok(Ethernet(tx,rx)) => (tx,rx),
        Ok(_) => panic!("unhandled channel type"),
        Err(e) => panic!("error occured {}",e),
    };
    (sender, reciever)

}
pub async fn setup_interface(inter_name: &str) -> NetworkInterface 
{
    let interface_names_match = |iface: &NetworkInterface | iface.name == inter_name; 
    let interfaces = datalink::interfaces(); 
    if let Some(interface) = interfaces.into_iter()
                              .filter(interface_names_match)
                              .next()
    {
        return interface
    }else{
        
    }
    interface
}
pub async fn get_send_recv(iface: &NetworkInterface ) -> packet_comm 
{

} 
pub async fn get_mac_single(iface: &NetworkInterface)
{


    if let Some(mac) = iface.mac {
        println!("{}" ,mac);
    }

}
// 
pub async fn scan() 
{
    for i in 0..255{


    }
} 
fn get_single_packet()
{


}
fn send_single_packet()
{

}
