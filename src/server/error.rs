use std::{fmt::Display, io::Error, sync::mpsc::RecvError};
use crate::server::protocol::error::ProtocolError;
/// An enum representing various types of errors that can occur in the application.
///
/// # Variants
///
/// - `AddressBindError`: Indicates a stream could not bind to its host:port
/// - `StreamAcceptError`: Indicates that an incoming stream could not have been accepted
/// - `StreamReadError`: Indicates that data could not be read from data stream 
/// - `ProtocolError`: Error associated with protocol create, read and update operations
/// - `ThreadError`: Error associated with multithreaded operations

pub enum ServerError {
     AddressBindError(Error),
     StreamAcceptError(Error),
     StreamReadError(Error),
     ProtocolError(ProtocolError),
     ThreadError(ThreadError)
}

///
/// # Variants
/// 
/// - `ChannelReceiveError`: Error associated with  [std::sync::mpsc::Receiver] channel transaction
/// - `ChannelSenderError`: Error associated with [std::sync::mpsc::Sender] channel transactions
/// 

#[derive(Debug)]
pub enum ThreadError {
     ChannelReceiveError(RecvError),
     ChannelSendError(RecvError),
}

/// Display implementation for ServerError
impl Display for ServerError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AddressBindError(e)=>{
               write!(f,"{{ error: AddressBindError; info: {} }}",e)
            },
            Self::ProtocolError(e)=>{
               write!(f, "{{ error: ProtocolError; info: {} }}", e)
            },
            Self::StreamAcceptError(e)=>{
               write!(f, "{{ error: StreamAcceptError; info: {} }}", e)
            },
            Self::ThreadError(e)=>{
               write!(f,"{{ error: ThreadError; info: {} }}",e )
            },
            Self::StreamReadError(e)=>{
               write!(f, "{{ error: StreamReadError; info: {} }}", e)
            }
        }
    }
}

/// Display implementation for ThreadError
impl Display for ThreadError{
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
          match self {
               Self::ChannelReceiveError(e)=>{
                    write!(f, "{{ error: ChannelReceiveError; info: {} }}", e)
               },
               Self::ChannelSendError(e)=>{
                    write!(f, "{{ error: ChannelSendError; info: {}  }}", e)
               }
     
          }
     }
}

/// Debug implementation for ServerError
impl std::fmt::Debug for ServerError{
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
         write!(f,"{}", self)
     }
 }