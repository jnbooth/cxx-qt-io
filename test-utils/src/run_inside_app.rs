use std::any::Any;
use std::mem;
use std::panic::{self, UnwindSafe};
use std::pin::Pin;
use std::time::Duration;

use crate::TestContext;

#[allow(clippy::module_inception)]
#[cxx::bridge]
mod ffi {
    extern "Rust" {
        type RunInsideAppContext;
    }

    #[namespace = "rust::cxxqtio1::test"]
    unsafe extern "C++" {
        include!("cxx-qt-io-test-utils/test_utils.h");

        type TestContext = crate::test_context::ffi::TestContext;

        fn run_inside_app(
            context: &mut RunInsideAppContext,
            functor: fn(&mut RunInsideAppContext, Pin<&mut TestContext>) -> bool,
        ) -> bool;
    }
}

struct RunInsideAppContext {
    #[allow(clippy::type_complexity)]
    closure: Option<Box<dyn FnOnce(TestContext) + UnwindSafe>>,
    error: Option<Box<dyn Any + Send>>,
}

impl RunInsideAppContext {
    fn run(&mut self, context: Pin<&mut ffi::TestContext>) -> bool {
        let context = unsafe { TestContext::new(context) };
        let closure = self.closure.take().unwrap();
        context.timeout_after(Duration::from_millis(10000));

        self.error = panic::catch_unwind(move || closure(context)).err();
        self.error.is_none()
    }
}

pub fn run_inside_app<F: FnOnce(TestContext) + UnwindSafe + 'static>(functor: F) {
    let app = cxx_qt_lib::QCoreApplication::new();
    let mut context = RunInsideAppContext {
        closure: Some(Box::new(functor)),
        error: None,
    };
    let completed = ffi::run_inside_app(&mut context, RunInsideAppContext::run);
    mem::drop(app);
    if let Some(error) = context.error.take() {
        panic::resume_unwind(error);
    }
    if !completed {
        panic!("test timed out");
    }
}
