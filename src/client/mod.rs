use std::{io::Write, net::TcpStream};

pub fn def_client(){
     let mut  c = TcpStream::connect("localhost:5000").expect("Something went wrong while client tried to connect to server");
     c.write("RECEIVE".as_bytes()).expect("sOMETHING WENT WRONG");
}