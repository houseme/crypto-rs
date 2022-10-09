use core::fmt::{self, Display, Formatter};

use alloc::string::FromUtf8Error;

#[cfg(feature = "std")]
use std::io::Error as IOError;

#[cfg(feature = "std")]
use std::error::Error;

use base64::DecodeError;

/// Errors for Crypto.
#[derive(Debug)]
pub enum CryptoError {
    #[cfg(feature = "std")]
    IOError(IOError),
    Base64Error(DecodeError),
    StringError(FromUtf8Error),
    DecryptError(BlockModeError),
}

#[cfg(feature = "std")]
impl From<IOError> for CryptoError {
    #[inline]
    fn from(error: IOError) -> CryptoError {
        CryptoError::IOError(error)
    }
}

impl From<DecodeError> for CryptoError {
    #[inline]
    fn from(error: DecodeError) -> CryptoError {
        CryptoError::Base64Error(error)
    }
}

impl From<FromUtf8Error> for CryptoError {
    #[inline]
    fn from(error: FromUtf8Error) -> CryptoError {
        CryptoError::StringError(error)
    }
}

impl From<BlockModeError> for CryptoError {
    #[inline]
    fn from(error: BlockModeError) -> CryptoError {
        CryptoError::DecryptError(error)
    }
}

impl Display for CryptoError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            #[cfg(feature = "std")]
            CryptoError::IOError(err) => Display::fmt(err, f),
            CryptoError::Base64Error(err) => Display::fmt(err, f),
            CryptoError::StringError(err) => Display::fmt(err, f),
            CryptoError::DecryptError(err) => Display::fmt(err, f),
        }
    }
}

#[cfg(feature = "std")]
impl Error for CryptoError {}
