use std::io; 
use std::io::{Write};
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
";
fn init_backend()
{

} 
pub fn start_cli()
{
    init_backend();
    println!("{}\n This project was created by Jaden Velasco show some love by starring the repo\n", ASCII);
    println!(" type help for documentation"); 
    loop {
    let mut usr_input = String::new(); 
        print!(">>>");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut usr_input)
            .expect("failed to read user input");
        match_usr_input(&usr_input.trim()); 
    }; 
} 
fn match_usr_input(usr_input: &str)
{
    let output = match usr_input{
        "help" =>  HELP,
//        "scan" => scan_cli(), 
        _ => "type help for commands\n"
    };
    print!("{}",output); 
}
fn scan_cli() 
{

} 
