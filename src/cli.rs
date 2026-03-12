use std::io; 
use std::io::{Write};
mod sock; 
use pnet::datalink::{self, NetworkInterface};
use pnet::packet::ethernet::EthernetPacket;
const ASCII: &str = r#"
                                                                                     
                                                        ,,                           
`7MM"""Yb.                   `7MM          `7MM"""Mq.   db                           
  MM    `Yb.                   MM            MM   `MM.                               
  MM     `Mb  ,6"Yb.  `7Mb,od8 MM  ,MP'      MM   ,M9 `7MM  `7MMpMMMb.  .P"Ybmmm     
  MM      MM 8)   MM    MM' "' MM ;Y         MMmmdM9    MM    MM    MM :MI  I8       
  MM     ,MP  ,pm9MM    MM     MM;Mm         MM         MM    MM    MM  WmmmP"       
  MM    ,dP' 8M   MM    MM     MM `Mb.       MM         MM    MM    MM 8M            
.JMMmmmdP'   `Moo9^Yo..JMML. .JMML. YA.    .JMML.     .JMML..JMML  JMML.YMMMMMb      
                                                                       6'     dP     
                                                                       Ybmmmd'        
"#;
const HELP: &str = "
    1) scan - scans the network for active hosts to dark ping. \n 
    2) interace <new_interface name> - change the interface which you are listening on\n
";

async fn get_interface(iface: &NetworkInterface) -> Option<String>
{

    println!("suggest interface is {}", iface.name);
    loop{
    let mut usr_input = String::new(); 
        println!("is this interface okay yes/no? ");
        print!(">>> ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut usr_input)
            .expect("failed to read user input");
        let check = usr_input.to_lowercase(); 
        if "no" ==check.trim()  
        {
            println!("please specify by doing --- interface <the one you wish to use>");
            let mut usr_input = String::new(); 
            print!(">>> ");
            io::stdout().flush().unwrap();
            io::stdin()
                .read_line(&mut usr_input)
                .expect("failed to read user input");
            return Some(usr_input.trim().to_string());
        } 
        else if "yes" ==check.trim(){
            return Some(iface.name.clone());
        } 
    }
    
}
pub async fn start_cli()
{
    
    let mut iface: NetworkInterface;
    let mut interface_name: String; 
    let mut iface: NetworkInterface; 
    let suggest_iface = sock::suggested_interface().await.unwrap();
    println!("{}\n This project was created by Jaden Velasco show some love by starring the repo\n", ASCII);
    println!(" type help for documentation"); 
    loop{
    interface_name = get_interface(&suggest_iface).await.unwrap()
    if let Some(iface) = iface = sock::setup_interface(&interface_name).await{
        break; 
    }else {
        continue; 
    }
    }
    //comms is the mpi for packets or if paralell is enabled
    loop {
    let mut usr_input = String::new(); 
        print!(">>> ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut usr_input)
            .expect("failed to read user input");
        match_usr_input(&usr_input.trim(), &comms); 
    }; 
} 
fn match_usr_input(usr_input: &str )
{
    scan_cli(usr_input, &comms); 
    let output = match usr_input{
        "help" =>  HELP,
        "scan" => scan_cli(usr_input,&comms), 
        _ => "type help for commands\n"
    };
    print!("{}",output); 
}

fn scan_cli(input: &str, comms: &sock::comm_tup<Option<EthernetPacket>>) -> str 
{

} 
