use std::{fmt::Debug, net::TcpStream};
///Trait to implement middlewares in the call stack
#[cfg(feature="developement")]
pub trait Middleware:Debug{
     fn intercept(incoming:TcpStream)->TcpStream;
}

