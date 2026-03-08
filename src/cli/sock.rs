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
/*pub async fn get_mpi<T>(iface: & NetworkInterface) -> mpsc_comm<T>
{ 

    let (mut sender, mut receiver)= match datalink::channel(&iface, Default::default()){
        Ok(Ethernet(tx,rx)) => (tx,rx),
        Ok(_) => panic!("unhandled channel type"),
        Err(e) => panic!("error occured {}",e),
    };

    let (tx_send, mut rx_send) = mpsc::channel(255);
    //lets use mpsc ONLY when we recieve a packet and want to send it to the master process 
    let rx_send_safe = Arc::new(Mutex::new(rex_send));
    let sender_safe = Arc::new(Mutex::new(sender));
   for _ in 0..(avaiable_parrellelism().unwrap().get()){ 
        let clone = rx_send_safe.clone();
        let clone_sender  = sender_safe.clone();
    tokio::spawn(async move {
        loop{
            let mut gaurd = clone.lock().await; 
            if let Some(pkt) = gaurd.recv().await{
                drop(gaurd);
                process(pkt).await;
            }
            
        }
        
    });
    }
    //receving packets
    (tx_send, rx_send)
    
}
*/
//pub async fn get_mpi<T>(iface: &NetworkInterface) -> comm_tup<T>
pub async sr()
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
    interface
}
/*
    let (mut sender, mut receiver)= match datalink::channel(&interface, Default::default()){
        Ok(Ethernet(tx,rx)) => (tx,rx),
        Ok(_) => panic!("unhandled channel type"),
        Err(e) => panic!("error occured {}",e),
 
 */
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
