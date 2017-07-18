use std::io;
use std::string::FromUtf8Error;

use protobuf::error::ProtobufError;

use message::Message;


#[derive(Debug)]
pub enum DemoError {
    Io(io::Error),
    Utf8(FromUtf8Error),
    Protobuf(ProtobufError),
    MagicMismatch(Box<[u8; 8]>),
    DemoProtocolMismatch(u32),
    GameDirectoryMismatch(String),
    UnknownCommand(u8),
    UnknownSplitFlags(u32),
    UnknownMessage(i32),
    ExpectedSendTable(Box<Message>),
}

impl From<io::Error> for DemoError {
    fn from(err: io::Error) -> DemoError {
        DemoError::Io(err)
    }
}

impl From<FromUtf8Error> for DemoError {
    fn from(err: FromUtf8Error) -> DemoError {
        DemoError::Utf8(err)
    }
}

impl From<ProtobufError> for DemoError {
    fn from(err: ProtobufError) -> DemoError {
        DemoError::Protobuf(err)
    }
}

pub type DemoResult<T> = Result<T, DemoError>;
