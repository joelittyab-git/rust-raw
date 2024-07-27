//Protocol transfer objects

///  A trait type for objects transferring data between threads, processes, etc.
/// 
///  # Type Parameters
///
/// * `I`: The type for sender identifer
/// * `U`: The type for sender (username)
/// * `B`: The type for sender body (data)
pub trait Proto<I,S,B> {
     /// Returns a unique identifier for the client.
     /// 
     /// # Returns
     /// * `I`: The type for sender identifer
     fn get_client_id(&self)->&I;

     /// Returns a unique identifier for the client seding the data.
     /// 
     /// # Returns
     /// * `U`: The type for sender (username)
     fn get_sender(&self)->&S;

     /// Returns a unique identifier for the client data is being sent to.
     /// 
     /// # Returns
     /// * `U`: The type for sender (username)
     fn get_receiver(&self)->&S;

     // Returns a unique identifier for the client.
     /// 
     /// # Returns
     /// * `B`: The type for body (data)
     fn get_body(&self)->&B;
}

///  A struct for implementing ProtocolTransferObject on BaseProtocol
///  Use this struct when data must be transfered between threads for listener client
///  and sender client when handling them in multiple threads 
/// 
///  # Fields
///
///  - `alias`: The unique identifier of the client (as a part of data in raw_bytes)
///  - `body`: The body of the data transmitted
#[derive(Debug)]
pub struct BaseProto{
     alias:String,
     body:String,
     to:String
}

impl Proto<String,String,String> for BaseProto{
     ///Returns the client id `SAME AS THE USERNAME`
     /// 
     /// # Returns
     /// - `String`: username
     fn get_client_id(&self)->&String {
          &self.alias
     }

     ///Returns the client id `SAME AS THE ID`
     /// 
     /// # Returns
     /// - `String`: username
     fn get_sender(&self)->&String {
          &self.alias
     }

     ///Returns the body of the data
     /// 
     /// # Returns
     /// - `String`: body
     fn get_body(&self)->&String {
          &self.body
     }

     ///Returns unize identifier of the client data is being sent to
     /// 
     /// # Returns
     /// - `String`: username
     fn get_receiver(&self)->&String {
         &self.to
     }
}