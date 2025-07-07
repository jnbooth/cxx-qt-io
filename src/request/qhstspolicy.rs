use std::fmt;
use std::mem::MaybeUninit;

use cxx::{type_id, ExternType};
use cxx_qt_lib::{QDateTime, QFlags, QString};

#[cxx::bridge]
mod ffi {
    #[repr(i32)]
    #[derive(PartialEq, Eq)]
    enum QHstsPolicyPolicyFlag {
        /// Indicates whether a policy must include subdomains.
        IncludeSubDomains = 1,
    }

    extern "C++" {
        include!("cxx-qt-lib/qdatetime.h");
        type QDateTime = cxx_qt_lib::QDateTime;
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    extern "C++" {
        include!("cxx-qt-io/qhstspolicy.h");
        type QHstsPolicyPolicyFlag;
        type QHstsPolicyPolicyFlags = super::QHstsPolicyPolicyFlags;
    }

    unsafe extern "C++" {
        type QHstsPolicy = super::QHstsPolicy;

        /// Returns the expiration date for the policy (in UTC).
        fn expiry(&self) -> QDateTime;

        /// Returns `true` if this policy also includes subdomains.
        #[rust_name = "includes_sub_domains"]
        fn includesSubDomains(&self) -> bool;

        /// Return `true` if this policy has a valid expiration date and this date is greater than [`QDateTime::current_date_time_utc()`](cxx_qt_lib::QDateTime::current_date_time_utc).
        #[rust_name = "is_expired"]
        fn isExpired(&self) -> bool;

        /// Sets the expiration date for the policy (in UTC) to `expiry`.
        #[rust_name = "set_expiry"]
        fn setExpiry(&mut self, expiry: &QDateTime);

        /// Sets whether subdomains are included for this policy to `include`.
        #[rust_name = "set_includes_sub_domains"]
        fn setIncludesSubDomains(&mut self, include: bool);
    }

    #[namespace = "rust::cxxqtio1"]
    unsafe extern "C++" {
        #[rust_name = "qhstspolicy_new"]
        fn qhstspolicyNew(
            expiry: &QDateTime,
            flags: QHstsPolicyPolicyFlags,
            host: &QString,
        ) -> QHstsPolicy;

        #[rust_name = "qhstspolicy_host"]
        fn qhstspolicyHost(policy: &QHstsPolicy) -> QString;

        #[rust_name = "qhstspolicy_set_host"]
        fn qhstspolicySetHost(policy: &mut QHstsPolicy, host: &QString);
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qhstspolicy_drop"]
        fn drop(policy: &mut QHstsPolicy);

        #[rust_name = "qhstspolicy_init_default"]
        fn construct() -> QHstsPolicy;
        #[rust_name = "qhstspolicy_clone"]
        fn construct(other: &QHstsPolicy) -> QHstsPolicy;

        #[rust_name = "qhstspolicy_eq"]
        fn operatorEq(a: &QHstsPolicy, b: &QHstsPolicy) -> bool;
    }
}

pub use ffi::QHstsPolicyPolicyFlag;

/// [`QFlags`] of [`QHstsPolicyPolicyFlag`].
pub type QHstsPolicyPolicyFlags = QFlags<QHstsPolicyPolicyFlag>;
unsafe_impl_qflag!(QHstsPolicyPolicyFlag, "QHstsPolicyPolicyFlags");

/// The `QHstsPolicy` class specifies that a host supports HTTP Strict Transport Security policy (HSTS).
///
/// Qt Documentation: [QHstsPolicy](https://doc.qt.io/qt-6/qhstspolicy.html#details)
#[repr(C)]
pub struct QHstsPolicy {
    _space: MaybeUninit<usize>,
}

impl Clone for QHstsPolicy {
    fn clone(&self) -> Self {
        ffi::qhstspolicy_clone(self)
    }
}

impl Default for QHstsPolicy {
    /// Constructs an invalid (expired) policy with empty host name and subdomains not included.
    fn default() -> Self {
        ffi::qhstspolicy_init_default()
    }
}

impl Drop for QHstsPolicy {
    fn drop(&mut self) {
        ffi::qhstspolicy_drop(self);
    }
}

impl PartialEq for QHstsPolicy {
    fn eq(&self, other: &Self) -> bool {
        ffi::qhstspolicy_eq(self, other)
    }
}

impl Eq for QHstsPolicy {}

impl fmt::Debug for QHstsPolicy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("QHstsPolicy")
            .field("expiry", &self.expiry())
            .field("host", &self.host())
            .field("includes_sub_domains", &self.includes_sub_domains())
            .finish()
    }
}

impl QHstsPolicy {
    /// Constructs `QHstsPolicy` with `expiry` (in UTC); `flags` is a value indicating whether this policy must also include subdomains.
    pub fn new(expiry: &QDateTime, flags: QHstsPolicyPolicyFlags, host: &QString) -> Self {
        ffi::qhstspolicy_new(expiry, flags, host)
    }

    /// Returns a host for a given policy.
    pub fn host(&self) -> QString {
        ffi::qhstspolicy_host(self)
    }

    /// Sets a host.
    pub fn set_host(&mut self, host: &QString) {
        ffi::qhstspolicy_set_host(self, host);
    }
}

// SAFETY: Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QHstsPolicy {
    type Id = type_id!("QHstsPolicy");
    type Kind = cxx::kind::Trivial;
}

#[cfg(test)]
mod tests {
    use cxx_qt_lib::QTimeZone;

    use super::*;

    #[test]
    fn props() {
        #[derive(Debug, PartialEq, Eq)]
        struct QHstsPolicyProps {
            expiry: QDateTime,
            host: QString,
            includes_sub_domains: bool,
        }

        let props = QHstsPolicyProps {
            expiry: QDateTime::from_secs_since_epoch(100_000.into(), &QTimeZone::utc()),
            host: QString::from("www.host.com"),
            includes_sub_domains: true,
        };

        let mut policy = QHstsPolicy::default();

        policy.set_expiry(&props.expiry);
        policy.set_host(&props.host);
        policy.set_includes_sub_domains(props.includes_sub_domains);

        let actual_props = QHstsPolicyProps {
            expiry: policy.expiry(),
            host: policy.host(),
            includes_sub_domains: policy.includes_sub_domains(),
        };

        assert_eq!(actual_props, props);
    }
}
