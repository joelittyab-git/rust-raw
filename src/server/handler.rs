use std::{io::Read, net::TcpStream, thread::spawn};


use crate::server::protocol::BaseProtocol;
use super::{error::ServerError, protocol::{Data, DataTransferProtocol}};


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
pub struct StreamHandler<P:DataTransferProtocol>{
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
     pub fn default_new(mut tcp_stream:TcpStream, service:TransmitService)->Result<StreamHandler<BaseProtocol>, ServerError>{
          //buffer data extraction from stream
          let mut buf = [0;1024];
          match tcp_stream.read(&mut buf){
               Err(e)=>return Err(ServerError::StreamReadError(e)),
               _=>()
          }
     
          //initializes base protocol using buffer data from the passed stream
          let proto = match BaseProtocol::new(Data::Utf8(buf)){
               Ok(t)=>{t},
               Err(e)=>{
                    return Err(ServerError::ProtocolError(e));
               }
          };
     
          Ok(StreamHandler{
               stream:tcp_stream,
               transmit:service,
               protocol:proto
          })
     }

     pub fn handle(&self){
          match self.transmit {
               TransmitService::Send=>self.handle_client_send(),
               TransmitService::Receive=>self.handle_client_receive()
          };
     }

     fn handle_client_send(&self){
          let handle = spawn(||{

          });
     }

     fn handle_client_receive(&self){
          todo!();
     }
}


