pub mod error;
pub mod pto;

use error::ProtocolError;
use log::warn;
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
}

pub struct ParsedData{
     /*Format-----------------------
     <alias>-<to>(/n)
     <body>
      ------------------------------*/

     raw:Data,
     to:String,
     alias:String,
     body:String
}

/// A trait for working which parsed data
/// A parsed data type implements the below and should contain the mthods provided 
/// to uniquely identify the sender and receiver and the content of the data
pub trait DataTransferProtocolParsed{
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

/// A trait for working with streams tranferring specific data containing data body and client identifier (alias)
///
/// This trait requires a parse method to be implemented which results in a type containing data 
/// which implements [DataTransferProtocolParser]
pub trait DataTransferProtocol {
     type Parsed: DataTransferProtocolParsed;
     fn parse(&self, data:Data)->Result<Self::Parsed, ProtocolError>;
}

/// An enum representing various encodings of data that can be sent through stream
/// ['Utf8'] encodes all unicode caharacters
/// ['Utf16'] encodes one or two 16-bit code units to represent each character.
/// This is a enum to handle actual raw data in the form of any two encodings
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

/// An enum representing various encodings of data that can be sent through stream
/// ['Utf8'] encodes all unicode caharacters
/// ['Utf16'] encodes one or two 16-bit code units to represent each character.
/// This is an enum represent what type of encoding does a handler handle
///
/// # Variants
///
/// - [`Utf8`]: Indicates a stream could not bind to its host:port
/// - [`Utf16`]: Indicates that an incoming stream could not have been accepted
/// 
/// ['Utf16']: Data::Utf16
/// ['Utf8']: Data::Utf8
pub enum DataType {
    Utf8,
    Utf16
}

impl BaseProtocol{
     /// Creates a new `BaseProtocol` with the given raw data in either of the encodings [`Utf8`] or [`Utf16`]
     ///
     /// # Errors
     ///
     /// This method might return a protocol error if invalid format for the protoco is present
     pub fn new()->BaseProtocol{
          Self{}
     }
}

impl DataTransferProtocol for BaseProtocol{
          
     type Parsed = ParsedData;
     /// Parses data and results in a parsed data type
     /// 
     /// # Arguments
     /// - `data` of type [Data] which contains raw bytes of encoding [DataType::Utf8] or [DataType::Utf16]
     fn parse(&self, data:Data)->Result<ParsedData, ProtocolError>{
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
          
          Ok(ParsedData{
               raw:data,
               to:to.to_string(),
               alias:alias.to_string(),
               body:body.to_string()
          })
     }

}

///Implementation of DataTransferProtocol trait for BaseProtocol
impl DataTransferProtocolParsed for ParsedData{
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

/**
 * I. SEND/RECEIVE
     - The client initializes a handshake by specifying the client type to the server

     /*Format-----------------------
     <type(SEND;<to-username>/RECEIVE<self-usrname>)>
      ------------------------------*/
 */
///Method to parse the handshake request, to identify the client as [TransmitService::Send] or [TransmitService::Receive]
pub fn get_type_for(raw:&[u8])->Result<TransmitService, ProtocolError>{
     //slizcing data for converting to string

     // let raw_send = &raw[0..4];
     // let raw_receive = &raw[0..7];

     //parsing raw to string
     let raw_parsed_send = &String::from_utf8_lossy(raw).trim().replace("\n", "")[0..4];
     let raw_parsed_receive = &String::from_utf8_lossy(raw).trim().replace("\n", "")[0..7];

     if raw_parsed_send=="SEND"{
          let mut raw_vec = raw.to_vec();
          raw_vec.retain(|&x| x!=0);
          let raw_string = String::from_utf8_lossy(&raw_vec);

          //unpacking data to extract username from handshake data
          let mut username = match raw_string.split_once(";"){
               None=>{return Err(ProtocolError::FromatError("Could not find ';' delemiter while extracting username from handshake data".to_string()))},
               Some((_,b))=>b.to_string()
          };
          username = username.trim().replace("\n", "");

          return Ok(TransmitService::Send(username));
     }else if  raw_parsed_receive=="RECEIVE" {
          //converting array to vector
          let mut raw_vec = raw.to_vec();
          //retain all non null values
          raw_vec.retain(|&x| x!=0);
          //converting to string
          let raw_string = String::from_utf8_lossy(&raw_vec);

          //unpacking data to extract username from handshake data
          let mut to_username = match raw_string.split_once(";"){
               None=>{return Err(ProtocolError::FromatError("Could not find ';' delemiter while extracting username from handshake data".to_string()))},
               Some((_,b))=>b.to_string()
          };
          to_username = to_username.trim().replace("\n", "");

          return Ok(TransmitService::Receive(to_username));
     }

     Err(ProtocolError::SessionExtractionError("Could not determine wether the session was send or receive.".to_string()))

}
