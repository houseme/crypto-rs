/*!
# Crypto-Rs
MagicCrypt is a Java/PHP/NodeJS/Rust library to encrypt/decrpyt strings, files, or data, using Data Encryption Standard(DES) or Advanced Encryption Standard(AES) algorithms. It supports CBC block cipher mode, PKCS5 padding and 64, 128, 192 or 256-bits key length.
## For Rust
### Example
```rust
use crypto_rs::{new_crypto, CryptoTrait};
let mc = new_crypto!("magickey", 256);
let base64 = mc.encrypt_str_to_base64("http://magiclen.org");
assert_eq!("DS/2U8royDnJDiNY2ps3f6ZoTbpZo8ZtUGYLGEjwLDQ=", base64);
assert_eq!("http://magiclen.org", mc.decrypt_base64_to_string(&base64).unwrap());
```
## Change the Buffer Size
The default buffer size for the `encrypt_reader_to_writer` method and the `decrypt_reader_to_writer` method is 4096 bytes. If you want to change that, you can use the `encrypt_reader_to_writer2` method or the `decrypt_reader_to_writer2` method, and define a length explicitly.
For example, to change the buffer size to 256 bytes,
```rust
use std::io::Cursor;
use crypto_rs::{new_crypto, CryptoTrait};
use crypto_rs::generic_array::typenum::U256;
let mc = new_crypto!("magickey", 256);
# #[cfg(feature = "std")] {
let mut reader = Cursor::new("http://magiclen.org");
let mut writer = Vec::new();
mc.encrypt_reader_to_writer2::<U256>(&mut reader, &mut writer).unwrap();
let base64 = base64::encode(&writer);
assert_eq!("DS/2U8royDnJDiNY2ps3f6ZoTbpZo8ZtUGYLGEjwLDQ=", base64);
assert_eq!("http://magiclen.org", mc.decrypt_base64_to_string(&base64).unwrap());
# }
```
## No Std
Disable the default features to compile this crate without std.
```toml
[dependencies.crypto_rs]
version = "*"
default-features = false
```
## For Java
Refer to https://github.com/magiclen/MagicCrypt.
## For PHP
Refer to https://github.com/magiclen/MagicCrypt.
## For NodeJS
Refer to https://github.com/magiclen/node-magiccrypt
 */

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

mod ciphers;
mod errors;
mod functions;
mod macros;
mod secure_bit;
mod traits;

use alloc::vec::Vec;

#[cfg(feature = "std")]
use std::io::{Read, Write};
#[cfg(feature = "std")]
use std::ops::Add;

pub use ciphers::aes128::Crypto128;
pub use ciphers::aes192::Crypto192;
pub use ciphers::aes256::Crypto256;
pub use ciphers::des64::Crypto64;
pub use digest::generic_array;
pub use errors::CryptoError;
pub use secure_bit::SecureBit;
pub use traits::CryptoTrait;

#[cfg(feature = "std")]
use generic_array::typenum::{IsGreaterOrEqual, PartialDiv, True, B1, U16};
#[cfg(feature = "std")]
use generic_array::ArrayLength;

#[derive(Debug, Clone)]
enum CryptoCipher {
    DES64(Crypto64),
    AES128(Crypto128),
    AES192(Crypto192),
    AES256(Crypto256),
}

/// This struct can help you encrypt or decrypt data in a quick way.
#[derive(Debug, Clone)]
pub struct Crypto {
    cipher: CryptoCipher,
}

impl Crypto {
    /// Create a new `Crypto` instance. You may want to use the `new_magic_crypt!` macro.
    pub fn new<S: AsRef<str>, V: AsRef<str>>(key: S, bit: SecureBit, iv: Option<V>) -> Crypto {
        let cipher = match bit {
            SecureBit::Bit64 => CryptoCipher::DES64(Crypto64::new(key, iv)),
            SecureBit::Bit128 => CryptoCipher::AES128(Crypto128::new(key, iv)),
            SecureBit::Bit192 => CryptoCipher::AES192(Crypto192::new(key, iv)),
            SecureBit::Bit256 => CryptoCipher::AES256(Crypto256::new(key, iv)),
        };

        Crypto { cipher }
    }
}

impl CryptoTrait for Crypto {
    #[inline]
    fn new<S: AsRef<str>, V: AsRef<str>>(key: S, iv: Option<V>) -> Crypto {
        Crypto::new(key, SecureBit::default(), iv)
    }

    #[inline]
    fn encrypt_to_bytes<T: ?Sized + AsRef<[u8]>>(&self, data: &T) -> Vec<u8> {
        match &self.cipher {
            CryptoCipher::DES64(mc) => mc.encrypt_to_bytes(data),
            CryptoCipher::AES128(mc) => mc.encrypt_to_bytes(data),
            CryptoCipher::AES192(mc) => mc.encrypt_to_bytes(data),
            CryptoCipher::AES256(mc) => mc.encrypt_to_bytes(data),
        }
    }

    #[cfg(feature = "std")]
    #[inline]
    fn encrypt_reader_to_bytes(&self, reader: &mut dyn Read) -> Result<Vec<u8>, CryptoError> {
        match &self.cipher {
            CryptoCipher::DES64(mc) => mc.encrypt_reader_to_bytes(reader),
            CryptoCipher::AES128(mc) => mc.encrypt_reader_to_bytes(reader),
            CryptoCipher::AES192(mc) => mc.encrypt_reader_to_bytes(reader),
            CryptoCipher::AES256(mc) => mc.encrypt_reader_to_bytes(reader),
        }
    }

    #[cfg(feature = "std")]
    #[inline]
    fn encrypt_reader_to_writer2<
        N: ArrayLength<u8> + PartialDiv<U16> + IsGreaterOrEqual<U16, Output = True>,
    >(
        &self,
        reader: &mut dyn Read,
        writer: &mut dyn Write,
    ) -> Result<(), CryptoError> {
        match &self.cipher {
            CryptoCipher::DES64(mc) => mc.encrypt_reader_to_writer2::<N>(reader, writer),
            CryptoCipher::AES128(mc) => mc.encrypt_reader_to_writer2::<N>(reader, writer),
            CryptoCipher::AES192(mc) => mc.encrypt_reader_to_writer2::<N>(reader, writer),
            CryptoCipher::AES256(mc) => mc.encrypt_reader_to_writer2::<N>(reader, writer),
        }
    }

    #[inline]
    fn decrypt_bytes_to_bytes<T: ?Sized + AsRef<[u8]>>(
        &self,
        bytes: &T,
    ) -> Result<Vec<u8>, CryptoError> {
        match &self.cipher {
            CryptoCipher::DES64(mc) => mc.decrypt_bytes_to_bytes(bytes),
            CryptoCipher::AES128(mc) => mc.decrypt_bytes_to_bytes(bytes),
            CryptoCipher::AES192(mc) => mc.decrypt_bytes_to_bytes(bytes),
            CryptoCipher::AES256(mc) => mc.decrypt_bytes_to_bytes(bytes),
        }
    }

    #[cfg(feature = "std")]
    #[inline]
    fn decrypt_reader_to_bytes(&self, reader: &mut dyn Read) -> Result<Vec<u8>, CryptoError> {
        match &self.cipher {
            CryptoCipher::DES64(mc) => mc.decrypt_reader_to_bytes(reader),
            CryptoCipher::AES128(mc) => mc.decrypt_reader_to_bytes(reader),
            CryptoCipher::AES192(mc) => mc.decrypt_reader_to_bytes(reader),
            CryptoCipher::AES256(mc) => mc.decrypt_reader_to_bytes(reader),
        }
    }

    #[cfg(feature = "std")]
    #[inline]
    fn decrypt_reader_to_writer2<
        N: ArrayLength<u8> + PartialDiv<U16> + IsGreaterOrEqual<U16, Output = True> + Add<B1>,
    >(
        &self,
        reader: &mut dyn Read,
        writer: &mut dyn Write,
    ) -> Result<(), CryptoError>
    where
        <N as Add<B1>>::Output: ArrayLength<u8>,
    {
        match &self.cipher {
            CryptoCipher::DES64(mc) => mc.decrypt_reader_to_writer(reader, writer),
            CryptoCipher::AES128(mc) => mc.decrypt_reader_to_writer(reader, writer),
            CryptoCipher::AES192(mc) => mc.decrypt_reader_to_writer(reader, writer),
            CryptoCipher::AES256(mc) => mc.decrypt_reader_to_writer(reader, writer),
        }
    }
}
