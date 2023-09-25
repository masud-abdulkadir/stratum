use std::{
    convert::From,
    fmt::Debug,
    sync::{MutexGuard, PoisonError},
};

use roles_logic_sv2::parsers::Mining;

#[derive(std::fmt::Debug)]
pub enum JdsError {
    Io(std::io::Error),
    ChannelSend(Box<dyn std::marker::Send + Debug>),
    ChannelRecv(async_channel::RecvError),
    BinarySv2(binary_sv2::Error),
    Codec(codec_sv2::Error),
    Noise(noise_sv2::Error),
    RolesLogic(roles_logic_sv2::Error),
    Framing(codec_sv2::framing_sv2::Error),
    PoisonLock(String),
    Custom(String),
    Sv2ProtocolError((u32, Mining<'static>)),
}

impl std::fmt::Display for JdsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use JdsError::*;
        match self {
            Io(ref e) => write!(f, "I/O error: `{:?}", e),
            ChannelSend(ref e) => write!(f, "Channel send failed: `{:?}`", e),
            ChannelRecv(ref e) => write!(f, "Channel recv failed: `{:?}`", e),
            BinarySv2(ref e) => write!(f, "Binary SV2 error: `{:?}`", e),
            Codec(ref e) => write!(f, "Codec SV2 error: `{:?}", e),
            Framing(ref e) => write!(f, "Framing SV2 error: `{:?}`", e),
            Noise(ref e) => write!(f, "Noise SV2 error: `{:?}", e),
            RolesLogic(ref e) => write!(f, "Roles Logic SV2 error: `{:?}`", e),
            PoisonLock(ref e) => write!(f, "Poison lock: {:?}", e),
            Custom(ref e) => write!(f, "Custom SV2 error: `{:?}`", e),
            Sv2ProtocolError(ref e) => {
                write!(f, "Received Sv2 Protocol Error from upstream: `{:?}`", e)
            }
        }
    }
}

pub type JdsResult<T> = Result<T, JdsError>;

impl From<std::io::Error> for JdsError {
    fn from(e: std::io::Error) -> JdsError {
        JdsError::Io(e)
    }
}

impl From<async_channel::RecvError> for JdsError {
    fn from(e: async_channel::RecvError) -> JdsError {
        JdsError::ChannelRecv(e)
    }
}

impl From<binary_sv2::Error> for JdsError {
    fn from(e: binary_sv2::Error) -> JdsError {
        JdsError::BinarySv2(e)
    }
}

impl From<codec_sv2::Error> for JdsError {
    fn from(e: codec_sv2::Error) -> JdsError {
        JdsError::Codec(e)
    }
}

impl From<noise_sv2::Error> for JdsError {
    fn from(e: noise_sv2::Error) -> JdsError {
        JdsError::Noise(e)
    }
}

impl From<roles_logic_sv2::Error> for JdsError {
    fn from(e: roles_logic_sv2::Error) -> JdsError {
        JdsError::RolesLogic(e)
    }
}

impl<T: 'static + std::marker::Send + Debug> From<async_channel::SendError<T>> for JdsError {
    fn from(e: async_channel::SendError<T>) -> JdsError {
        JdsError::ChannelSend(Box::new(e))
    }
}

impl From<String> for JdsError {
    fn from(e: String) -> JdsError {
        JdsError::Custom(e)
    }
}
impl From<codec_sv2::framing_sv2::Error> for JdsError {
    fn from(e: codec_sv2::framing_sv2::Error) -> JdsError {
        JdsError::Framing(e)
    }
}

impl<T> From<PoisonError<MutexGuard<'_, T>>> for JdsError {
    fn from(e: PoisonError<MutexGuard<T>>) -> JdsError {
        JdsError::PoisonLock(e.to_string())
    }
}

impl From<(u32, Mining<'static>)> for JdsError {
    fn from(e: (u32, Mining<'static>)) -> Self {
        JdsError::Sv2ProtocolError(e)
    }
}
