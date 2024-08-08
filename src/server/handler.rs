use std::{io::{Read, Write}, net::TcpStream, sync::{mpsc::{Receiver, Sender}, Arc, Mutex, MutexGuard}};
use log::{error, info, warn};


use crate::server::protocol::res::{Response, Status};
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
     /// `If handler reads 0 data from stream buffer it disconnects from client stream`
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
                    _=>{
                         info!("Read data");
                    }
               };

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
               let client_chx_sender = match self.search_rcp_sender_for(username, &rcp){
                    Some(sender)=>sender,
                    None=>{
                         let s = "error getting sender".as_bytes();
                         self.stream.write(s).expect("Something went wrong while printing error message back to client");
                         continue;
                    }
               };

               //unpacking parsed data
               let body = match parsed.get_body(){
                    Ok(body)=>body.to_string(),
                    Err(e)=>{
                         warn!("Could not parse body {{{:?}}}",e);
                         continue;
                    }
               };
               let to = parsed.get_to().to_string();
               let alias = parsed.get_client_id().to_string();

               //Base proto instance creation to transfer data through channel
               let pto = BaseProto::create(alias, body, to);

               //sending data through channel
               if let Err(e) = client_chx_sender.send(pto){
                    error!("Error sending data though stream from sender to receiver thread {{{:?}}}", e);
               };

               let res = Response::generate_res(Status::Success, "The message has been dispatched from sender handler".to_string());
               match self.stream.write(res.as_bytes()){
                    Err(e)=>{
                         error!("Error occured while sending response status to client {{{e}}}")
                    },
                    _=>()
               };
               info!("Message has been dispactched to {{ username: {username} }} thread listener...");

          }
     }


     /// Handles [TransmitService::Receive] type client 
     /// 
     /// # Arguments
     /// - `chx`: A [std::sync::mpsc::Receiver<T>] object associated with a channel. Since this method handles [TransmitService::Receive] type clients it awaits for 
     ///            incoming data from a [std::sync::mpsc::Sender<T>] obejct associated with some other thread stored in the [crate::server] 
     ///            pool of [crate::server::container::ClientSenderContainer]
     pub fn handle_client_receive(&mut self, chx:Receiver<BaseProto>)->Result<(), ServerError>{
          warn!("Received and handling receive");
          loop {
               let pto = match chx.recv(){
                    Err(e)=>{
                         return Result::Err(ServerError::ThreadError(ThreadError::ChannelReceiveError(e)));
                    },
                    Ok(t)=>{
                         warn!("Received message");
                         t
                    }
               };

               //extracting usernake from prtocol transfer object
               let username = pto.get_receiver().to_owned();

               //attempting to convert pto to raw bytes
               let raw = match self.protocol.to_raw(pto){
                    Ok(byte_vec)=>byte_vec,
                    Err(e)=>{
                         error!("Error converting pto to raw bytes in handle_client_receive {{{:?}}}",e);
                         continue;
                    }
               };

               //writes to receive client stream
               match self.stream.write(&raw){
                    Err(e)=>{
                         error!("Error writing {{ {} }}", e);
                    },
                    _=>()
               };

               //logs
               info!("Successfully written to {{ username: {}; type: RECEIVE }}", username)
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