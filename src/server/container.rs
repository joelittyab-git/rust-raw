use std::fmt::Display;
use std::sync::mpsc::{Sender, Receiver};
use std::thread:: JoinHandle;


/// A struct representing a thread-stream container
/// contains instance of thread for the handling of incoming stream (listens to data being sent to the server),
/// and thread handle associated with the handlers of the incoming data
/// Contains the chanel for the thread which sends data to client receiver thread
/// 
/// Defualt channel_rx is None since the client sender attaches send data through the sender in client cnotainer
/// Multiple sender instances can connect to the sender channel instance.
/// # Diagram
/// 
/// ```text
/// +-----------------------+   
/// |      Client  (sn)     |   
/// +-----------+-----------+   
///             |              
///             V             
/// +-----------------------+       +---------+------------+           +---------------Pool-----------------+
/// |        Server         | ----> |    Thread Creation   |  ----->   | get_sender_for(username)->Sender   |
/// +-----------+-----------+       +---------+------------+           +------------------------------------+
///             |                                                                         |    
///             |                                                                         |
///             |                                                                         V
///             |                                                              +-----------------------+   
///             |                                                              |      Sender.send()    |   
///             |                                                              +-----------+-----------+
///         _________
///         |       |
///         |Receive|
///         |_______| 
/// ```
/// # Fields
///
/// - `id`: A unique identifier of a specific container
/// - `thread_handle`: The thread to handle incoming data from stream
/// - `channel_rx`: The receiver object of the channel(sender object) initialized in thread created
/// - `to_alias`: The uniuqe identifier of the client to which the stream this container handles
#[derive(Debug)]
pub struct  ClientSenderContainer<T>{
     id:u64,
     thread_handle:JoinHandle<()>,   //thread handle for the incoming request listener 
     channel_rx:Option<Receiver<T>>,
     to_alias:String
}

/// A struct representing a thread-stream container
/// contains instance of thread for the handling of outgoing stream (send data to the client),
/// This container only handles tcp streams 
/// This container contains instance of the Sender object associated with the channel moved in the 
/// handling thread
/// 
/// # Diagram
/// 
/// ```text
/// +-----------------------+   
/// |      Client  (rx)     |   
/// +-----------+-----------+   
///             |              
///             V             
/// +-----------------------+       +---------+------------+           
/// |        Server         | ----> |    Thread Creation   |  ----->   await_for_data()
/// +-----------+-----------+       +---------+------------+           
/// ```
/// 
/// # Fields
///
/// - `id`: A unique identifier of a specific container
/// - `thread_handle`: The thread to handle incoming data from stream
/// - `channel_tx`: The Sender object of the channel(Receiver) initialized and sent to the thread
/// - `alias`: The unique identifier of the client stream registered in this container
#[derive(Debug)]
pub struct ClientReceiverContainer<T>{
     id:u64,
     thread_handle:JoinHandle<()>,
     channel_tx:Option<Sender<T>>,
     alias:String
}

impl <'s,T>ClientReceiverContainer<T> {
     /// Defacult constructor for the ClientReceiverContainer instance
     /// 
     /// # Arguments
     /// 
     /// * `handle`: JoinHandle<()> of the thread running a handler
     /// * `channel_sender`: Sender<T> of the channel associated with the Receiver<T> in the executing in the thread
     /// * `key`: Unique key for this container instance
     /// * `alias`: The unique identifier of the client
     pub fn new(handle:JoinHandle<()>, channel_sender:Sender<T>, key:u64, alias:String)->Self{
          ClientReceiverContainer{
               id:key,
               thread_handle:handle,
               channel_tx:Some(channel_sender),
               alias
          }
     }

     /// Destroys the container objec
     /// Do this before thread handle goes out of scope
     pub fn drop(self){
          drop(self);
     }

     //----Getters----
     pub fn get_id(&self)->u64{
          self.id
     }

     pub fn get_sender(&self)->Option<Sender<T>>{
          return match &self.channel_tx{
               None=>None,
               Some(channel)=>Some(channel.clone())
          }
     }

     pub fn get_thread_handle(&self)->&JoinHandle<()>{
          &self.thread_handle
     }

     pub fn get_alias(&self)->&String{
          &self.alias
     }
}



impl <T>ClientSenderContainer<T> {
     /// Defacult constructor for the ClientReceiverContainer instance
     /// 
     /// # Arguments
     /// 
     /// * `handle`: JoinHandle<()> of the thread running a handler
     /// * `channel_sender`: Sender<T> of the channel associated with the Receiver<T> in the executing in the thread
     /// * `key`: Unique key for this container instance
     /// * `to_alias`: The unique identifier of the client to which the stream registered in this container sends to
     pub fn new(handle:JoinHandle<()>, channel_receiver:Receiver<T>, key:u64, to_alias:String)->Self{
          ClientSenderContainer{
               id:key,
               thread_handle:handle,
               channel_rx:Some(channel_receiver),
               to_alias
          }
     }

     /// Destroys the container objec
     /// Do this before thread handle goes out of scope
     pub fn drop(self){
          drop(self);
     }

     //----Getters----
     pub fn get_id(&self)->u64{
          self.id
     }

     pub fn get_receiver(&mut self)->Option<Receiver<T>>{


          return self.channel_rx.take();
     }

     pub fn get_thread_handle(&self)->&JoinHandle<()>{
          &self.thread_handle
     }

     pub fn get_alias(&self)->&String{
          &self.to_alias
     }
}

/// Display implementation for ClientReceiverContainer
impl <T>Display for ClientReceiverContainer<T> {
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
          write!(f, "{{ id: {}; alias: {}; type: RECEIVE }}", self.id, self.alias)
     }
}

/// Display implementation for ClientSenderContainer
impl <T>Display for ClientSenderContainer<T>{
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
          write!(f, "{{ id: {}; to_alias: {}; type: SEND }}", self.id, self.to_alias)
     }
}
