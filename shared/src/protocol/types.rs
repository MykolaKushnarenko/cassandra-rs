use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum Flags {
    None =          0x00,
    Compression =   0x01,
    Tracing =       0x02,
    CustomPayload = 0x04,
    Warning =       0x08
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum Version {
    Request = 0x04,
    Response = 0x84
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum Opcode {
    Error = 0x00,
    Startup = 0x01,
    Ready = 0x02,
    Authenticate = 0x03,
    Options = 0x05,
    Supported      = 0x06,
    Query          = 0x07,
    Result         = 0x08,
    Prepare        = 0x09,
    Execute        = 0x0A,
    Register       = 0x0B,
    Event          = 0x0C,
    Batch          = 0x0D,
    AuthChallenge  = 0x0E,
    AuthResponse   = 0x0F,
    AuthSuccess    = 0x10,
}

impl From<u8> for Opcode {
    fn from(opcode: u8) -> Self {
        match opcode {
            0x00 => Opcode::Error,
            0x01 => Opcode::Startup,
            0x02 => Opcode::Ready,
            0x03 => Opcode::Authenticate,
            0x05 => Opcode::Options,
            0x06 => Opcode::Supported,
            0x07 => Opcode::Query,
            0x08 => Opcode::Result,
            0x09 => Opcode::Prepare,
            0x0A => Opcode::Execute,
            0x0B => Opcode::Register,
            0x0C => Opcode::Event,
            0x0D => Opcode::Batch,
            0x0E => Opcode::AuthChallenge,
            0x0F => Opcode::AuthResponse,
            0x10 => Opcode::AuthSuccess,
            _ => panic!("Invalid opcode")
        }
    }
}

impl From<u8> for Version {
    fn from(version: u8) -> Self {
        match version {
            0x04 => Version::Request,
            0x84 => Version::Response,
            _ => panic!("Invalid version")
        }
    }
}

impl From<u8> for Flags {
    fn from(version: u8) -> Self {
        match version {
            0x00 => Flags::None,
            0x01 => Flags::Compression,
            0x02 => Flags::Tracing,
            0x04 => Flags::CustomPayload,
            0x08 => Flags::Warning,
            _ => panic!("Invalid flags")
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Request {
    Add(String),
    Check(String)
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Response {
    String(String),
    Array(Vec<String>),
    Bool(bool)
}