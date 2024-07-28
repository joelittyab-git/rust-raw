use std::{io::Read, net::TcpStream, sync::mpsc::{Receiver, Sender}, thread::spawn};


use log::warn;

use crate::server::protocol::BaseProtocol;
use super::{error::ServerError, protocol::{pto::Proto, Data, DataTransferProtocol, DataTransferProtocolParsed}};


/// A struct representing a stream handler
/// Handles a stream exclusiive to one transmit type:['Send'] or ['Receive']
/// Handles streams that are specific to one protocol
///
/// # Fields
///
/// - `stream`: the TcpStream that this handler object handles ['TransmitService']
/// - `transmit`: The transmit service subscribed by the client
/// = 'protocol': The protocol type followed by this handler which implements ['DataTransferProtocol']
/// 
/// ['TransmitService']: TransmitService
/// ['Send']: TransmitService::Send
/// ['Receive']: TransmitService::Receive
/// ['DataTransferProtocol']: crate::server::protocol::DataTransferProtocol
pub struct StreamHandler<P>
where P:DataTransferProtocol{
     stream:TcpStream,
     transmit:TransmitService,
     protocol: P
}


/// Represents a service type for a client.
/// 
/// Each client subscribes to one of the service for which they respresent.
/// 1. Receive only Client
/// 2. Send only Client
///
/// # Variants
///
/// - `Sender`: Respresents a client that only sends data.
/// - `Receive`: Represents a client that only receives data.

pub enum TransmitService{
    Send,
    Receive
}

impl <P:DataTransferProtocol> StreamHandler<P>{
     /// creates a new handler object to handle a client
     /// 
     /// 
     /// # Arguments
     ///
     /// * `tcp_stream` - initlialized tcp stream of type ['TcpStream']
     /// * `protocol` - A protocol that implements ['DataTransferProtocol']
     /// * `service` - A Transmit service for the respective stream [SEND] or [RECEIVE]
     /// 
     /// ['DataTransferProtocol']: crate::server::protocol::DataTransferProtocol
     /// ['TcpStream']: std::net::TcpStream
     /// [SEND]: TransmitService::Send
     /// [RECEIVE]: TransmitService::Receive
     /// 
     pub fn new(tcp_stream:TcpStream, protocol:P, service:TransmitService)->Result<Self, ServerError>{
     
          Ok(Self{
               stream:tcp_stream,
               transmit:service,
               protocol:protocol
          })
     }

     /// Handles [TransmitService::Send] type client 
     /// 
     /// # Arguments
     /// - `chx`: A [std::sync::mpsc::Sender<T>] object associated with a channel. Since this method handles [TransmitService::Send] type clients it awaits for 
     ///            incoming data in streams to send to the Receiver type stored in [crate::server] pool
     ///            Type `<T>` should be a pto object that implements Proto to transfer data between threads
     pub fn handle_client_send<T,A,B,C>(&self, chx:Sender<T>)
     where T:Proto<A,B,C,>{
          warn!("Received and handling send");

          loop {
              
          }
     }


     /// Handles [TransmitService::Receive] type client 
     /// 
     /// # Arguments
     /// - `chx`: A [std::sync::mpsc::Receiver<T>] object associated with a channel. Since this method handles [TransmitService::Receive] type clients it awaits for 
     ///            incoming data from a [std::sync::mpsc::Sender<T>] obejct associated with some other thread stored in the [crate::server] 
     ///            pool of [crate::server::container::ClientSenderContainer]
     pub fn handle_client_receive<T,A,B,C>(&self, chx:Receiver<T>)
     where T:Proto<A,B,C>{
          warn!("Received and handling receive");
          loop {
              
          }
     }
}

/// creates a new handler object to handle a client by using default protocol ['BaseProtocol']
/// 
/// 
/// # Arguments
///
/// * `tcp_stream` - initlialized tcp stream of type ['TcpStream']
/// * `service` - A Transmit service for the respective stream [SEND] or [RECEIVE]
/// 
/// ['BaseProtocol']: BaseProtocol
/// ['TcpStream']: std::net::TcpStream
/// [SEND]: TransmitService::Send
/// [RECEIVE]: TransmitService::Receive
/// 
/// # Returns 
/// * `Result<StreamHandler<BaseProtocol>, ServerError>`
/// 
pub fn default_new(tcp_stream:TcpStream, service:TransmitService)->Result<StreamHandler<BaseProtocol>, ServerError>{
     Ok(StreamHandler{
          stream:tcp_stream,
          transmit:service,
          protocol:BaseProtocol::new()
     })
}

///Clone implementation for Transmit Service
impl Clone for TransmitService {
     fn clone(&self) -> Self {
          match self {
               Self::Send => Self::Send,
               Self::Receive => Self::Receive,
          }
     }
}