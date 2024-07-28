use std::net::TcpStream;

pub trait Middleware{
     fn intercept(incoming:TcpStream)->TcpStream;
}