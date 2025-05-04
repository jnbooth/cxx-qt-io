use std::any::Any;
use std::mem;
use std::panic::{self, UnwindSafe};

use cxx_qt_io::QEventLoop;

pub fn run_inside_app<F>(closure: F)
where
    F: FnOnce() + UnwindSafe,
{
    let app = cxx_qt_lib::QCoreApplication::new();
    let mut event_loop = QEventLoop::new();
    if let Err(error) = event_loop.pin_mut().try_exec_with(closure) {
        panic::resume_unwind(error);
    }
    mem::drop(app);
}
