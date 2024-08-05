use std::{io::Error, sync::mpsc::RecvError};
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

#[derive(Debug)]
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
