#[derive(thiserror::Error, Debug)]
pub enum PacketError {
    #[error("{} is not a valid IP address", .0)]
    InvalidAddress(String),
    #[error("data not available because packet was not built")]
    PacketNotBuilt,
    #[error("failed to read data from packet")]
    ReadError(#[from] std::io::Error),
    #[error("failed to decode data with WINDOWS_1251 instance")]
    Windows1251Error,
    #[error("failed to decode data to utf-8")]
    Utf8Error(#[from] std::string::FromUtf8Error),
}
