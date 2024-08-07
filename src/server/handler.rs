use std::{any::Any, io::{Read, Write}, net::TcpStream, process::{exit, ExitCode}, rc, sync::{mpsc::{Receiver, Sender}, Arc, Mutex, MutexGuard}};


use log::{error, warn};

use crate::server::protocol::BaseProtocol;
use super::{container::ClientReceiverContainer, error::{ServerError,ThreadError}, protocol::{pto::{BaseProto, Proto}, Data, DataTransferProtocol, DataTransferProtocolParsed}};

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
where P:DataTransferProtocol<String,String,String>{
     stream:TcpStream,
     transmit:TransmitService,
     protocol: P,
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
#[derive(Debug)]
pub enum TransmitService{
    Send(String),
    Receive(String)
}

impl <P:DataTransferProtocol<String,String,String>> StreamHandler<P>{
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
     pub fn handle_client_send<T,A,B,C>(&mut self, chx:Sender<T>, rcp:Arc<Mutex<Vec<ClientReceiverContainer<BaseProto>>>>)
     where T:Proto<A,B,C,>{
          warn!("Received and handling send");

          loop {
               //buffer to read input data
               let mut buf:[u8;1024] = [0;1024];

               //handles if the reading of stream returns an error
               match self.stream.read(&mut buf){
                    Err(e)=>{
                         error!("An error occured while reading stream {{{}}}", e);
                         continue;
                    },
                    Ok(0)=>{       //handles disconnected stream, when 0 data is read
                         warn!("Stream has disconnected");
                         break;
                    },
                    _=>()
               };
               println!("{:?}", buf);

               //parses read data
               let parsed = match self.protocol.parse(Data::Utf8(buf)){
                    Err(e)=>{
                         error!("An error occured while parsing protocol {{{:?}}}", e);
                         continue;
                    },
                    Ok(s)=>s
               };

               //extracts data from parsed
               let username = parsed.get_to();

               //rcp search for parsed username
               //arc clone and locking to read data
               let cloned_rcp:Arc<Mutex<Vec<ClientReceiverContainer<BaseProto>>>> = rcp.clone();
               let rcp:MutexGuard<Vec<ClientReceiverContainer<BaseProto>>> = cloned_rcp.lock().unwrap();
               let cli_sender = match self.search_rcp_sender_for(username, &rcp){
                    Some(sender)=>sender,
                    None=>{
                         todo!();
                         // self.stream.write(buf)
                    }
               };

               

          }

          // //buffer to read input data
          // let mut buf:[u8;1024] = [0;1024];

          // //handles if the reading of stream returns an error
          // println!("Waiting to read data");
          // if let Err(e) = self.stream.read(&mut buf){
          //      error!("An error occured while reading stream {{{}}}", e);
          // };
          // println!("{:?}", buf);

          // //parses read data
          // let parsed = match self.protocol.parse(Data::Utf8(buf)){
          //      Err(e)=>{
          //           error!("An error occured while parsing protocol {{{:?}}}", e);
          //           exit(1);
          //      },
          //      Ok(s)=>s
          // };

          // //extracts data from parsed
          // let username = parsed.get_to();

          // //rcp search for parsed username
          // //arc clone and locking to read data
          // let cloned_rcp:Arc<Mutex<Vec<ClientReceiverContainer<BaseProto>>>> = rcp.clone();
          // let rcp:MutexGuard<Vec<ClientReceiverContainer<BaseProto>>> = cloned_rcp.lock().unwrap();
          // let cli_sender = match self.search_rcp_sender_for(username, &rcp){
          //      Some(sender)=>sender,
          //      None=>{
          //           todo!();
          //           // self.stream.write(buf)
          //      }
          // };
     }


     /// Handles [TransmitService::Receive] type client 
     /// 
     /// # Arguments
     /// - `chx`: A [std::sync::mpsc::Receiver<T>] object associated with a channel. Since this method handles [TransmitService::Receive] type clients it awaits for 
     ///            incoming data from a [std::sync::mpsc::Sender<T>] obejct associated with some other thread stored in the [crate::server] 
     ///            pool of [crate::server::container::ClientSenderContainer]
     pub fn handle_client_receive(&self, chx:Receiver<BaseProto>)->Result<(), ServerError>{
          warn!("Received and handling receive");
          loop {
               let pto = match chx.recv(){
                    Err(e)=>{
                         return Result::Err(ServerError::ThreadError(ThreadError::ChannelReceiveError(e)));
                    },
                    Ok(t)=>t
               };

               let raw = self.protocol.to_raw(pto);
               //todp
          }
     }

     fn search_rcp_sender_for<X>(&self,username:&String, rcp:&Vec<ClientReceiverContainer<X>>)->Option<Sender<X>>
     where X:Proto<String,String,String>{
          for crp in rcp{
               if crp.get_alias().to_string()==username.to_string(){
                    return crp.get_sender();
               }
          }

          return None;
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
               Self::Send(s) => Self::Send(s.clone()),
               Self::Receive(s) => Self::Receive(s.clone()),
          }
     }
}