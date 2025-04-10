#[cxx::bridge]
mod ffi {
    /// Describes the key types for alternative name entries in [`QSslCertificate`](crate::QSslCertificate).
    #[repr(i32)]
    enum QSslAlternativeNameEntryType {
        /// An email entry; the entry contains an email address that the certificate is valid for.
        EmailEntry,
        /// A DNS host name entry; the entry contains a host name entry that the certificate is valid for. The entry may contain wildcards.
        DnsEntry,
        /// An IP address entry; the entry contains an IP address entry that the certificate is valid for.
        IpAddressEntry,
    }

    extern "C++" {
        include!("cxx-qt-io/qssl.h");
        type QSslAlternativeNameEntryType;
    }
}

pub use ffi::QSslAlternativeNameEntryType;
