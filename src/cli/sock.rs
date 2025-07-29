extern crate pnet; 
use pnet::datalink::{self, NetworkInterface};
use pnet::datalink::Channel::Ethernet;
use pnet::packet::{Packet, MutablePacket};
use std::thread; 
use tokio::sync::{mpsc, Mutex, OnceCell}; 
use std::collections::HashMap; 
pub async fn suggested_interface() -> Option<NetworkInterface>
{
    let res= datalink::interfaces()
        .into_iter()
        .find(|iface: &NetworkInterface| !iface.is_loopback() && iface.is_up() && iface.mac.is_some() );
    res
}
pub async fn get_hashmap()
{

} 
pub async fn get_mpi(sender: &mut DataLinkSender, reciver: &mut DatalinkReciever) -> &(Sender<T>,Reciever<T>)
{
    let once: OnceCell<(Sender<T>, Reciever<T>)>= OnceCell::new(); 
    once.get_or_init(|| async{ 
    let once = mpsc::channel(64);
    let (tx, rx) = &once; 
    std::spawn( move ||async{
        loop{

        }
         
    }); 
    std::spawn( move ||async{
        loop{
            

        }
    }); 
    return once;
    ).await;

}
pub async fn setup_interface(inter_name: &str) 
{
    let interface_names_match = |iface: &NetworkInterface | iface.name == inter_name; 
    let interfaces = datalink::interfaces(); 
    let interface = interfaces.into_iter()
                              .filter(interface_names_match)
                              .next()
                              .unwrap(); 
    let (mut sender, mut reciever)= match datalink::channel(&interface, Default::default()){
        Ok(Ethernet(tx,rx)) => (tx,rx),
        Ok(_) => panic!("unhandled channel type"),
        Err(e) => panic!("error occured {}",e),
    };

}
pub async fn scan() 
{

} 
fn get_single_packet()
{


}
fn send_single_packet()
{

}
