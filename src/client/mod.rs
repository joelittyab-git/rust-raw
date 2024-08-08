use std::{io::{Read, Write}, net::TcpStream, thread, time::Duration};

pub fn def_client(){
     let mut  c = TcpStream::connect("localhost:5000").expect("Something went wrong while client tried to connect to server");
     c.write("SEND;scale".as_bytes()).expect("sOMETHING WENT WRONG");
     let str_buf = format!("scale-rand\n\rSome data is here");
     let buf = str_buf.as_bytes();
     thread::sleep(Duration::from_secs(6));
     c.write(buf).expect("Something went wrong while sending data");
     thread::sleep(Duration::from_secs(10));
     let mut  read_buf = [0;1028];
     c.read(&mut read_buf).expect("Something went wrong while reading from server...");
     println!("{}", String::from_utf8_lossy(&read_buf));
     c.shutdown(std::net::Shutdown::Both).expect("Something went wrong while trying to shutdown stream");
}

pub fn create_client(username:String){
     let mut  c = TcpStream::connect("localhost:5000").expect("Something went wrong while client tried to connect to server");
     let x = format!("RECEIVE;{username}");
     c.write(x.as_bytes()).expect("sOMETHING WENT WRONG");
     thread::sleep(Duration::from_secs(10));
     let mut  read_buf = [0;1028];
     c.read(&mut read_buf).expect("Something went wrong while reading from server...");
     println!("{}", String::from_utf8_lossy(&read_buf));
     read_buf = [0;1028];
     println!("{}", String::from_utf8_lossy(&read_buf));
     c.read(&mut read_buf).expect("Something went wrong while reading from server...");
     c.shutdown(std::net::Shutdown::Both).expect("Something went wrong while trying to shutdown stream");
}