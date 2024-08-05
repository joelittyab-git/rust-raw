enum Status {
    Success,
    InvalidIdentifier,
    ServerError
}

struct Response;
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