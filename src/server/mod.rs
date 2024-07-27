/// A server service to handle ['transmit_service']
/// 
/// ['transmit_service']:handler::TransmitService

pub mod protocol;
pub mod error;
pub mod handler;
pub mod container;         //Thread-stream container

use std::{io::Read, net::{
     TcpListener,
     TcpStream
}, process::exit, thread};
use log::{info,error};

use error::ServerError;
use container::{ClientReceiverContainer, ClientSenderContainer};
use protocol::pto::BaseProto;
use handler::{StreamHandler, TransmitService};
use protocol::{BaseProtocol, get_type_for};


/// A struct representing a [Server] instance that binds on an endpoint anc
/// accepts incoming stream requests and handles them using the [StreamHandler].
/// It is implemented to use the [BaseProtocol] for transferring data and its pto [BaseProto]
///
/// # Fields
///
/// - `host`: The host on which the server is hosted
/// - `port`: The port on which the server is posted
/// - `stream``: The pool record of incoming streams
/// - `send_container_pool`: Or scp, a pool of [ClientSenderContainer], contains the pool of active running send client thread handles and their channels
/// - `receive_container_pool`: Or rcp, a pool of [ClientReceiverContainer], contains the pool of active running receive client thread handles and their channels
#[derive(Debug)]
pub struct Server<'a>{
     host:String,
     port:i32,
     stream:Vec<TcpStream>,
     send_container_pool:Vec<ClientSenderContainer<'a, BaseProto>>,
     receive_container_pool:Vec<ClientReceiverContainer<'a, BaseProto>>,
     stream_counter:i64       //maintains the id for each incoming stream

}


impl <'s> Server<'s>{
     /// Default constructor for the server. Use this constructor to initialize new [Server]
     /// 
     /// 
     /// # Arguments
     ///
     /// * `host` - the host on which the server has to run
     /// * `port` - The port on which the server should run on
     /// 
     pub fn new(host:String, port:i32)->Self{
          //container pool initialization
          let scp:Vec<ClientSenderContainer<'s, BaseProto>> = Vec::new();
          let rcp:Vec<ClientReceiverContainer<'s, BaseProto>> = Vec::new();


          info!("Initialized server.");
          let stream:Vec<TcpStream> = Vec::new();
          Server{
               host,
               port,
               stream,
               send_container_pool:scp,
               receive_container_pool:rcp,
               stream_counter:0
          }
     }

     pub fn serve(&mut self)->Result<(), ServerError>{
          //constructing address string from port number and host  
          let mut addr = self.host.trim()
               .to_string();
          addr.push(':');
          addr.push_str(self.port.to_string().as_str());
          info!("Server is initialized and is starting on \"{addr}\"");

          //initializing TcpListener
          let listener = match TcpListener::bind(addr){
               Ok(e)=>e,
               Err(e)=>return Err(ServerError::AddressBindError(e))
          };

          loop {
               //accepting incoming streams
               let (mut stream, addr) = match listener.accept(){
                    Ok(tas)=>tas,
                    Err(e)=>return Err(ServerError::StreamAcceptError(e))
               };

               let mut buf = [0;1024];

               //readining initial handshake request
               let client_type:TransmitService = match get_type_for(&mut buf){
                    Ok(t)=>t,
                    Err(e)=>{
                         error!("An error occured when type was being extracted from incoming stream {:?}", e);
                         exit(1);
                    }
               };

               //TODO:
               match client_type {
                   TransmitService::Receive=>{},
                   TransmitService::Send=>{}
               }

               //logging 
               let stream_id = &self.generate_id();
               info!("Incoming: {}", stream_id);

               //reading the type of client receive or send
               let mut buf = [0;1024];
               stream.read(&mut buf).expect("type read error");
               println!("{}",String::from_utf8_lossy(&buf));

               //container creation

          }

          Ok(())
     }

     fn generate_id(&mut self)->i64{
          self.stream_counter+=1;
          self.stream_counter
     }
}

