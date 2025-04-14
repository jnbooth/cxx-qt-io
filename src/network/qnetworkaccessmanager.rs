#[cxx::bridge]
mod ffi {
    /// Indicates the operation this reply is processing.
    #[repr(i32)]
    #[derive(Debug)]
    enum QNetworkAccessManagerOperation {
        /// retrieve headers operation (created with [`QNetworkAccessManager::head`](https://doc.qt.io/qt-6/qnetworkaccessmanager.html#head))
        HeadOperation = 1,
        /// retrieve headers and download contents (created with [`QNetworkAccessManager::get`](https://doc.qt.io/qt-6/qnetworkaccessmanager.html#get))
        GetOperation,
        /// upload contents operation (created with [`QNetworkAccessManager::put`](https://doc.qt.io/qt-6/qnetworkaccessmanager.html#put))
        PutOperation,
        /// send the contents of an HTML form for processing via HTTP POST (created with [`QNetworkAccessManager::post`](https://doc.qt.io/qt-6/qnetworkaccessmanager.html#post))
        PostOperation,
        /// delete contents operation (created with [`QNetworkAccessManager::delete_resource`](https://doc.qt.io/qt-6/qnetworkaccessmanager.html#deleteResource))
        DeleteOperation,
        /// custom operation (created with [`QNetworkAccessManager::send_custom_request`](https://doc.qt.io/qt-6/qnetworkaccessmanager.html#sendCustomRequest))
        CustomOperation,
    }

    extern "C++" {
        include!("cxx-qt-io/qnetworkaccessmanager.h");
        type QNetworkAccessManagerOperation;
    }
}

pub use ffi::QNetworkAccessManagerOperation;
