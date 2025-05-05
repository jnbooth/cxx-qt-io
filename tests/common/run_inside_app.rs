use std::any::Any;
use std::panic::{self, UnwindSafe};

use cxx_qt_io::QEventLoop;

use super::ensure_app_is_running;

pub fn run_inside_app<F>(closure: F)
where
    F: FnOnce() + UnwindSafe,
{
    ensure_app_is_running();
    let mut error: Option<Box<dyn Any + Send>> = None;
    QEventLoop::new().pin_mut().exec_with(|| {
        error = panic::catch_unwind(closure).err();
    });
    if let Some(error) = error {
        panic::resume_unwind(error);
    }
}
