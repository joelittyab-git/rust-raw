
/// An enum representing all the status codes that can be sent to the client
///
/// # Variants
///
/// - `Success`: Respresent a success message dispatch 
/// - `InvalidIdentifier`: Represents an invalid client identifier (username)
/// - `ServerError`: Server Error
pub enum Status {
    Success,
    InvalidIdentifier,
    ServerError
}

/// Struct for generating responses after client handles the message and sends the status code along with message
pub struct Response;
/*<Status>;<Message>*/

impl Response{
     pub fn generate_res(code:Status, message:String)->String{
          match code {
              Status::InvalidIdentifier=>format!("InvalidIdentifier;{}", message),
              Status::ServerError=>format!("ServerError;{}", message),
              Status::Success=>format!("Success;{}", message)
          }
     }
}