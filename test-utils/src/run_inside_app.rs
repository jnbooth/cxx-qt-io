use std::any::Any;
use std::mem;
use std::panic::{self, UnwindSafe};

#[allow(clippy::module_inception)]
#[cxx::bridge]
mod ffi {
    extern "Rust" {
        type RunInsideAppContext;
    }

    #[namespace = "rust::cxxqtio1::test"]
    unsafe extern "C++" {
        include!("cxx-qt-io-test-utils/test_utils.h");

        fn run_inside_app(
            context: &mut RunInsideAppContext,
            functor: fn(&mut RunInsideAppContext) -> bool,
        ) -> i32;
    }
}

struct RunInsideAppContext {
    closure: Option<Box<dyn FnOnce() + UnwindSafe>>,
    error: Option<Box<dyn Any + Send>>,
}

impl RunInsideAppContext {
    fn run(&mut self) -> bool {
        let closure = self.closure.take().unwrap();
        self.error = panic::catch_unwind(closure).err();
        self.error.is_none()
    }
}

#[track_caller]
pub fn run_inside_app<F: FnOnce() + UnwindSafe + 'static>(functor: F) {
    let app = cxx_qt_lib::QCoreApplication::new();
    let mut context = RunInsideAppContext {
        closure: Some(Box::new(functor)),
        error: None,
    };
    ffi::run_inside_app(&mut context, RunInsideAppContext::run);
    if let Some(error) = context.error.take() {
        panic::resume_unwind(error);
    }
    mem::drop(app);
}
