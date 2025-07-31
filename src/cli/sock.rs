extern crate pnet; 
use pnet::datalink::Channel::Ethernet;
use pnet::packet::{Packet, MutablePacket};
use tokio::sync::mpsc::{Sender, Receiver};
use pnet::datalink::{self, DataLinkSender, DataLinkReceiver, NetworkInterface};
use tokio::sync::{mpsc, Mutex, OnceCell}; 
use std::collections::HashMap; 


type mpsc_comm<T> = (Sender<T>, Receiver<T>);

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
/*pub async fn get_mpi<'a,T>(sender: &'a mut dyn DataLinkSender, reciver: &'a mut dyn DataLinkReceiver, iface: &'a NetworkInterface )-> mpsc_comm<T> 
{
    let once: = OnceCell::new();
    once.get_or_init(|| async{ 
    let (tx,rx)= mpsc::channel(64);
    tokio::spawn( async move{
        loop{
        }
         
    }); 
    tokio::spawn( async move{
        loop{
            

        }
    }); 
    (tx,rx)
    }).await
}*/

pub async fn setup_interface(inter_name: &str) -> NetworkInterface 
{
    let interface_names_match = |iface: &NetworkInterface | iface.name == inter_name; 
    let interfaces = datalink::interfaces(); 
    let interface = interfaces.into_iter()
                              .filter(interface_names_match)
                              .next()
                              .unwrap(); // need to add error handling here since we could unwrap
                                         // a none value leading to a crash because the user gave
                                         // an incorrect name
    let (mut sender, mut reciever)= match datalink::channel(&interface, Default::default()){
        Ok(Ethernet(tx,rx)) => (tx,rx),
        Ok(_) => panic!("unhandled channel type"),
        Err(e) => panic!("error occured {}",e),
    };
    interface
}
pub async fn get_mac_single(iface: &NetworkInterface)
{


    if let Some(mac) = iface.mac {
        println!("{}" ,mac);
    }

}
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
