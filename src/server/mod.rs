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
}, sync::{mpsc::{
     channel,
     Receiver,
     Sender
}, Arc, Mutex, MutexGuard},thread:: {sleep, spawn},
 time::Duration
};
use log::{error, info};

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
/// - `send_container_pool`: Or scp, a pool of [ClientSenderContainer], contains the pool of active running send client thread handles and their channels. Arc mutex to handle multi-threaded stream handling.
/// - `receive_container_pool`: Or rcp, a pool of [ClientReceiverContainer], contains the pool of active running receive client thread handles and their channels. Arc mutex to handle multi-threaded stream handling.
#[derive(Debug)]
pub struct Server{
     host:String,
     port:i32,
     send_container_pool:Arc<Mutex<Vec<ClientSenderContainer<BaseProto>>>>,
     receive_container_pool:Arc<Mutex<Vec<ClientReceiverContainer<BaseProto>>>>,
     stream_counter:u64,       //maintains the id for each incoming stream
     // middleware_pool:Vec<Box<dyn middleware::Middleware>>
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

          //initialiing shared mutable datasource for multithreaded stream handlers
          let rcp_shared:Arc<Mutex<Vec<ClientReceiverContainer<BaseProto>>>> = Arc::new(Mutex::new(rcp));
          let scp_shared:Arc<Mutex<Vec<ClientSenderContainer<BaseProto>>>> = Arc::new(Mutex::new(scp));

          info!("Initialized server.");
          Server{
               host,
               port,
               send_container_pool:scp_shared,
               receive_container_pool:rcp_shared,
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
               let mut handler:StreamHandler<BaseProtocol> =  match default_new(stream, client_service.clone()){
                    Ok(e)=>e,
                    Err(e)=>{
                         error!("Could not initialize stream handler due to... {}", e);
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
                              let _ =handler.handle_client_receive(receiver);
                         });
                         info!("Accepted incoming request from {addr} -- {{ id: {}; receive_alias: {} }}", key, s);          //logging
                         // container creation for this above handler and channel compoenents
                         let container = ClientReceiverContainer::new(handle, sender, key,s);
                         // cloning arc
                         let cloned_shared_rcp = self.receive_container_pool.clone();
                         // locking mutex
                         let mut rcp = cloned_shared_rcp.lock().unwrap();
                         rcp.push(container);

                    },
                    TransmitService::Send(to)=>{
                         let cloned_scp:Arc<Mutex<Vec<ClientReceiverContainer<BaseProto>>>> = self.receive_container_pool.clone();
                         let handle = spawn(move ||{
                              handler.handle_client_send(cloned_scp);
                         });
                         info!("Accepted incoming request from {addr} -- {{ id: {}; to_alias: {} }}", key, to);              //logging
                         //container creation for this above handler and channel compoenents
                         let container:ClientSenderContainer<BaseProto> = ClientSenderContainer::new(handle, receiver, key, to);
                         //cloning scp arc
                         let cloned_shared_scp:Arc<Mutex<Vec<ClientSenderContainer<BaseProto>>>> = self.send_container_pool.clone();
                         //locking scp mutex
                         let mut scp:MutexGuard<Vec<ClientSenderContainer<BaseProto>>> = cloned_shared_scp.lock().unwrap();
                         scp.push(container);
                         
                    }
               };

          }
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

