#[cxx::bridge]
mod ffi {
    #[repr(i32)]
    #[derive(Debug)]
    enum QSslSocketPeerVerifyMode {
        VerifyNone,
        QueryPeer,
        VerifyPeer,
        AutoVerifyPeer,
    }
}

pub use ffi::QSslSocketPeerVerifyMode;
