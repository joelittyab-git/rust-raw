pub mod error;
pub mod pto;

use error::ProtocolError;
use pto::Proto;

use super::handler::TransmitService;

/// A struct representing a protocol defining a structure of how data is transmited in a stream
///
/// # Fields
///
/// - `raw`: The raw data transmitted in a stream of type ['Utf8'] or ['Utf16']
/// - `alias`: The unique identifier of the client (as a part of data in raw_bytes)
/// - `to`: The client id of the client this data is being sent to
/// - `body`: The body of the data transmitted
/// ['Utf8']: Data::Utf8
/// ['Utf16']: Data::Utf16
pub struct BaseProtocol{

     /*Format-----------------------
     <alias>-<to>(/n)
     <body>
      ------------------------------*/

     raw:Data,
     to:String,
     alias:String,
     body:String
}

/// A trait for working with streams tranferring specific data containing data body and client identifier (alias)
///
/// This trait provides methods for obtaining a unique identifier for
/// a client and retrieving data in a stream format.
pub trait DataTransferProtocol{
     /// Returns a unique identifier for the client.
     /// 
     /// # Returns
     /// 
     /// Returns a 'String' that uniquely identifies a specfic client
     fn get_client_id(&self) -> &String;

     // Returns a unique identifier of the user data is being sent to
     /// 
     /// # Returns
     /// 
     /// Returns a 'String' that uniquely identifies a specfic client
     fn get_to(&self) -> &String;

     /// Returns the body of the data sent through stream
     /// 
     /// # Returns
     /// 
     /// Returns a 'Result<String, ProtocoError>' 
     /// returns the body of the data as string if success
     fn get_body(&self) -> Result<&String, ProtocolError>;    
}


/// An enum representing various encodings of data that can be sent through stream
/// ['Utf8'] encodes all unicode caharacters
/// ['Utf16'] encodes one or two 16-bit code units to represent each character.
///
/// # Variants
///
/// - [`Utf8`]: Indicates a stream could not bind to its host:port
/// - [`Utf16`]: Indicates that an incoming stream could not have been accepted
/// 
/// ['Utf16']: Data::Utf16
/// ['Utf8']: Data::Utf8
pub enum Data {
    Utf8([u8;1024]),
    Utf16([u16;1024])
}

impl BaseProtocol{
     /// Creates a new `BaseProtocol` with the given raw data in either of the encodings [`Utf8`] or [`Utf16`]
     /// # Arguments
     ///
     /// * `data` - The raw data in one of the encodings provided
     ///
     /// # Errors
     ///
     /// This method might return a protocol error if invalid format for the protoco is present
     pub fn new(data:Data)->Result<Self, ProtocolError>{
          let raw_str = match data{
               Data::Utf16(d)=>{
                    String::from_utf16_lossy(&d).to_string()
               },
               Data::Utf8(d)=>{
                    String::from_utf8_lossy(&d).to_string()
               }
          };

          //splits the data according to the format of this protocol
          let (head, body ) = match raw_str.split_once('\n'){
               None=>return Err(ProtocolError::FromatError("Could not extract data and header from the data...".to_string())),
               Some(x)=>x
          };

          let (alias, to) = match  head.split_once("-"){
               None=>return Err(ProtocolError::FromatError("Could not extract alias and to".to_string())),
               Some(t)=>t
          };
          
          
          Ok(Self{
               raw:data,
               alias:alias.to_string(),
               body:body.to_string(),
               to:to.to_string() 
          })
     }


     ///----getters-----
     pub fn get_body(&self)->&String{
          &self.body
     }
     pub fn get_alias(&self)->&String{
          &self.alias
     }
}

///Implementation of DataTransferProtocol trait for BaseProtocol
impl DataTransferProtocol for BaseProtocol{
     /// # Returns:
     /// The body of the transfered data
     fn get_body(&self) -> Result<&String, ProtocolError> {
         Ok(&self.body)
     }
     /// # Returns:
     /// The identifier of the client sending the data
     fn get_client_id(&self) -> &String {
         &self.alias
     }
     /// # Returns:
     /// The identifier of the client receiving the data
     fn get_to(&self) -> &String {
         &self.to
     }
}



pub fn get_type_for(raw:&[u8])->Result<TransmitService, ProtocolError>{
     let raw_parsed = String::from_utf8_lossy(raw).to_string().trim().to_uppercase();

     return match raw_parsed.as_str() {
          "SEND"=>Ok(TransmitService::Send),
          "RECEIVE"=>Ok(TransmitService::Receive),
          _=>Err(ProtocolError::SessionExtractionError("Could not determine wether the session was send or receive.".to_string()))
     };
}
