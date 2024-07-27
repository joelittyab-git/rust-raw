/// An enum representing various types of errors that can occur in the application.
/// Protocol error during the execution of protocol related operations
/// 
///
/// # Variants
///
/// - `DataExtractionError`: Indicates that the body of a protocol could not be extracted
/// = 'FromatError': Indicates that the data parsed has not implemented the data protocol properly, ie.. data not formated properly

#[derive(Debug)]
pub enum ProtocolError {
    SessionExtractionError(String),
    BodyExtractionError(String),
    FromatError(String),
}