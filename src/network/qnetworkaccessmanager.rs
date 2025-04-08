#[cxx::bridge]
mod ffi {
    /// Indicates the operation this reply is processing.
    #[repr(i32)]
    #[derive(Debug)]
    enum QNetworkAccessManagerOperation {
        /// retrieve headers operation (created with `head()`)
        HeadOperation = 1,
        /// retrieve headers and download contents (created with `get()`)
        GetOperation,
        /// upload contents operation (created with `put()`)
        PutOperation,
        /// send the contents of an HTML form for processing via HTTP POST (created with `post()`)
        PostOperation,
        /// delete contents operation (created with `delete_resource()`)
        DeleteOperation,
        /// custom operation (created with `send_custom_request()`)
        CustomOperation,
    }

    extern "C++" {
        include!("cxx-qt-io/qnetworkaccessmanager.h");
        type QNetworkAccessManagerOperation;
    }
}

pub use ffi::QNetworkAccessManagerOperation;
