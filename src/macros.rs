/// This macro provides a convenient way to create a `Crypto<bits>` instance or a `Crypto` instance.
#[macro_export]
macro_rules! new_crypto {
    (wrapper $key:expr) => {
        $crate::Crypto::new($key, $crate::SecureBit::Bit128, None::<String>)
    };
    (wrapper $key:expr,64) => {
        $crate::Crypto::new($key, $crate::SecureBit::Bit64, None::<String>)
    };
    (wrapper $key:expr,128) => {
        $crate::Crypto::new($key, $crate::SecureBit::Bit128, None::<String>)
    };
    (wrapper $key:expr,192) => {
        $crate::Crypto::new($key, $crate::SecureBit::Bit192, None::<String>)
    };
    (wrapper $key:expr,256) => {
        $crate::Crypto::new($key, $crate::SecureBit::Bit256, None::<String>)
    };
    (wrapper $key:expr,64, $iv:expr) => {
        $crate::Crypto::new($key, $crate::SecureBit::Bit64, Some($iv))
    };
    (wrapper $key:expr,128, $iv:expr) => {
        $crate::Crypto::new($key, $crate::SecureBit::Bit128, Some($iv))
    };
    (wrapper $key:expr,192, $iv:expr) => {
        $crate::Crypto::new($key, $crate::SecureBit::Bit192, Some($iv))
    };
    (wrapper $key:expr,256, $iv:expr) => {
        $crate::Crypto::new($key, $crate::SecureBit::Bit256, Some($iv))
    };
    ($key:expr) => {{
        use $crate::CryptoTrait;

        $crate::Crypto128::new($key, None::<String>)
    }};
    ($key:expr,64) => {{
        use $crate::CryptoTrait;

        $crate::Crypto64::new($key, None::<String>)
    }};
    ($key:expr,128) => {{
        use $crate::CryptoTrait;

        $crate::Crypto128::new($key, None::<String>)
    }};
    ($key:expr,192) => {{
        use $crate::CryptoTrait;

        $crate::Crypto192::new($key, None::<String>)
    }};
    ($key:expr,256) => {{
        use $crate::CryptoTrait;

        $crate::Crypto256::new($key, None::<String>)
    }};
    ($key:expr,64, $iv:expr) => {{
        use $crate::CryptoTrait;

        $crate::Crypto64::new($key, Some($iv))
    }};
    ($key:expr,128, $iv:expr) => {{
        use $crate::CryptoTrait;

        $crate::Crypto128::new($key, Some($iv))
    }};
    ($key:expr,192, $iv:expr) => {{
        use $crate::CryptoTrait;

        $crate::Crypto192::new($key, Some($iv))
    }};
    ($key:expr,256, $iv:expr) => {{
        use $crate::CryptoTrait;

        $crate::Crypto256::new($key, Some($iv))
    }};
}
