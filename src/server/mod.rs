/// A server service to handle ['transmit_service']
/// 
/// ['transmit_service']:handler::TransmitService

pub mod protocol;
pub mod middleware;
pub mod auth;
pub mod error;
pub mod handler;
pub mod container;         //Thread-stream container

use std::{io::Read, net::{
     TcpListener,
     TcpStream
}, process::exit, sync::mpsc::{channel, Receiver, Sender}, thread:: {spawn, sleep}, time::Duration};
use log::{error, info, warn};

use error::ServerError;
use container::{ClientReceiverContainer, ClientSenderContainer};
use handler::{StreamHandler, TransmitService, default_new};
use protocol::{BaseProtocol, get_type_for_raw_utf8, pto::BaseProto};


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
pub struct Server{
     host:String,
     port:i32,
     stream:Vec<TcpStream>,
     send_container_pool:Vec<ClientSenderContainer<BaseProto>>,
     receive_container_pool:Vec<ClientReceiverContainer<BaseProto>>,
     stream_counter:u64       //maintains the id for each incoming stream

}


impl Server{
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
          let scp:Vec<ClientSenderContainer<BaseProto>> = Vec::new();
          let rcp:Vec<ClientReceiverContainer<BaseProto>> = Vec::new();


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

     ///Starts serving at host port initailized while constructing the instance 
     /// Call this to run server
     pub fn serve(&mut self)->Result<(), ServerError>{
          //constructs address
          let addr = self.construct_addr();
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

               let client_service = match self.identify_request_type(&mut stream){
                    None=>{continue;},
                    Some(t)=>t

               };

               //handler creation to handle the incoming stream
               let handler:StreamHandler<BaseProtocol> =  match default_new(stream, client_service.clone()){
                    Ok(e)=>e,
                    Err(e)=>{
                         error!("Could not initialize stream handler due to... {:?}", e);
                         continue;
                    }
               };


               //channels creartion to communicate between streams in different thread
               let(sender, receiver):
                         (Sender<BaseProto>, Receiver<BaseProto>) = channel();


               let key = self.generate_id();      //key generation for container id

               //moving the handling of each stream to their handlers in separate threads
               match client_service {
                    TransmitService::Receive(s)=>{
                         let handle = spawn(move ||{
                              handler.handle_client_receive(receiver);
                         });
                         info!("Accepted incoming request from {addr} -- {{ id: {}; receive_alias: {} }}", key, s);          //logging
                         //container creation for this above handler and channel compoenents
                         let container = ClientReceiverContainer::new(handle, sender, key,s);
                         self.receive_container_pool.push(container);
                    },
                    TransmitService::Send(to)=>{
                         let handle = spawn(move ||{
                              handler.handle_client_send(sender);
                         });
                         info!("Accepted incoming request from {addr} -- {{ id: {}; to_alias: {} }}", key, to);              //logging
                         //container creation for this above handler and channel compoenents
                         let container = ClientSenderContainer::new(handle, receiver, key, to);
                         self.send_container_pool.push(container);
                         
                    }
               };

          }

          Ok(())
     }

     /// method to identify request type from stream data {initial handshake}
     fn identify_request_type(&self, tcp_stream:&mut TcpStream)->Option<TransmitService>{
          let mut buf = [0;1024];  //buffer to read initial handshake

          if let Err(e) = tcp_stream.read(&mut buf){
               error!("An error occured when type was being extracted from incoming stream {:?}", e);
               sleep(Duration::from_secs(1));     //thread sleep
               return None;
          }

          //readining initial handshake request
          let client_service = match get_type_for_raw_utf8(&mut buf){
               Ok(t)=>Some(t),
               Err(e)=>{
                    error!("An error occured when type was being extracted from incoming stream {:?}", e);
                    None
               }
          };

          return client_service;

          
     }

     fn generate_id(&mut self)->u64{
          self.stream_counter+=1;
          self.stream_counter
     }

     /// Method that costructs address from port number and address
     fn construct_addr(&self)->String{
          let mut addr = self.host.trim()
               .to_string();
          addr.push(':');
          addr.push_str(self.port.to_string().as_str());

          addr
     }
}

