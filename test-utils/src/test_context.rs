use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{mem, ptr};

use cxx::memory::UniquePtrTarget;
use cxx::UniquePtr;
use cxx_qt::{QObject, Upcast};

#[allow(clippy::module_inception)]
#[cxx::bridge]
pub mod ffi {
    extern "C++" {
        type QObject = cxx_qt::QObject;
    }

    #[namespace = "rust::cxxqtio1::test"]
    unsafe extern "C++" {
        include!("cxx-qt-io-test-utils/test_utils.h");

        type TestContext;

        fn exit(self: Pin<&mut Self>);
        unsafe fn hold(self: Pin<&mut Self>, object: *mut QObject);
        #[rust_name = "timeout_after"]
        fn timeoutAfter(self: Pin<&mut Self>, msecs: i32);
    }
}

/// Wrapper to implement `Send` on `*mut ffi::TestContext`.
struct TestContextPtr(*mut ffi::TestContext);
// SAFETY: ffi::TestContext has no interior mutability. Also, Mutex requires Send because of
// Mutex::into_inner, but that isn't used here.
unsafe impl Send for TestContextPtr {}

#[derive(Clone)]
pub struct TestContext {
    inner: Arc<Mutex<TestContextPtr>>,
}

impl TestContext {
    /// # Safety
    ///
    /// `value` must remain pinned for the lifetime of this object.
    pub unsafe fn new(value: Pin<&mut ffi::TestContext>) -> Self {
        // SAFETY: Upheld by constructor contract.
        let inner = unsafe { ptr::from_mut(Pin::into_inner_unchecked(value)) };
        Self {
            inner: Arc::new(Mutex::new(TestContextPtr(inner))),
        }
    }

    pub fn exit(&self) {
        self.with_pin(ffi::TestContext::exit);
    }

    pub fn hold<T>(&self, object: UniquePtr<T>)
    where
        T: UniquePtrTarget + Upcast<QObject>,
    {
        self.with_pin(|pin| {
            // SAFETY: `inner` will delete held objects at the end of the test.
            unsafe {
                let object = T::upcast_ptr(object.into_raw()).cast_mut();
                pin.hold(object);
            };
        });
    }

    pub fn timeout_after(&self, duration: Duration) {
        self.with_pin(|pin| {
            pin.timeout_after(i32::try_from(duration.as_millis()).unwrap_or(i32::MAX));
        });
    }

    fn with_pin<F>(&self, f: F)
    where
        F: FnOnce(Pin<&mut ffi::TestContext>),
    {
        let mut lock = self
            .inner
            .try_lock()
            .expect("concurrent TestContext access");
        // SAFETY: Upheld by constructor contract.
        f(unsafe { Pin::new_unchecked(&mut *lock.0) });
        mem::drop(lock);
    }
}
