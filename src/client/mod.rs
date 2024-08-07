use std::{io::{Read, Write}, net::TcpStream, thread, time::Duration};

pub fn def_client(){
     let mut  c = TcpStream::connect("localhost:5000").expect("Something went wrong while client tried to connect to server");
     c.write("SEND;scale".as_bytes()).expect("sOMETHING WENT WRONG");

     thread::sleep(Duration::from_secs(6));
}