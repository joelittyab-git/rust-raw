use std::net::TcpStream;

///Trait to implement middlewares in the call stack
#[cfg(feature="developement")]
pub trait Middleware{
     fn intercept(incoming:TcpStream)->TcpStream;
}