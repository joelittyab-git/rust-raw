use std::fmt::Display;

/// An enum representing various types of errors that can occur in the application.
/// Protocol error during the execution of protocol related operations
/// 
///
/// # Variants
///
/// - `DataExtractionError`: Indicates that the body of a protocol could not be extracted
/// - `FromatError`: Indicates that the data parsed has not implemented the data protocol properly, ie.. data not formated properly
#[derive(Debug)]
pub enum ProtocolError {
    SessionExtractionError(String),
    FromatError(String),
}

/// Display implementation for ProtocolError
impl Display for ProtocolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FromatError(e)=>{
                write!(f, "{{ error: FormatError; info: {} }}", e)
            },
            Self::SessionExtractionError(e)=>{
                write!(f, "{{ error: SessionExtractionError; info: {} }}", e)
            }
        }
    }
}