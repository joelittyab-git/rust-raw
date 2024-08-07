use std::io::stdin;
use std::u64::MAX;

use server::protocol::{self, Data, DataTransferProtocol};
use server::Server;
mod server;
mod client;

fn main(){
     // println!("{}", generate_key(&mut "max".to_string()));
     // println!("{}", MAX);

     // let x = "base-senior\n\rbody data is herer".as_bytes();
     // let mut y:[u8;1024] = [0;1024];

     // for i in 0..x.len(){
     //      y[i] = x[i];
     // }


     // let proto = protocol::BaseProtocol::new(Data::Utf8(y)).unwrap();
     // let alias = proto.get_alias();
     // let to = proto.get_to();
     // let b = proto.get_body();

     // println!("Content:{},\nAlias: {}\nTo: {}", b,alias,to);

     
     //initializing logger
     env_logger::builder().filter_level(log::LevelFilter::Info).init();

     let mut inp = String::new();
     stdin().read_line(&mut inp).expect("Something went wrong");

     if(inp.contains('a')){
          let mut server = Server::new("localhost".to_string(), 5000);
          server.serve().expect("seving went wrong");
     }else if inp.trim().replace("\n", "")=="s"{
          client::def_client();
     }else if inp.trim().replace("\n", "")=="r"{
          println!("Enter username: ");
          let mut buf = String::new();
          stdin().read_line(&mut buf).expect("Something went wrong while reading from stdin");
          buf = buf.trim().to_string();
          buf = buf.replace("\n", "");
          client::create_client(buf);
     }


     // let a = vec![83, 69, 78, 68,0,0,0,0,0,0,0,0,0,0,0,0];
     // println!("{}","SEND"=="SEND");
     // let raw_parsed = String::from_utf8_lossy(&a).trim().replace("\n", "");

     // println!("{}", "SEND"==raw_parsed);

}
