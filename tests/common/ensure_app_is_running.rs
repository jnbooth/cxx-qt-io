use std::sync::atomic::{AtomicBool, Ordering};

use cxx_qt_lib::QCoreApplication;

pub fn ensure_app_is_running() {
    static APP_EXISTS: AtomicBool = AtomicBool::new(false);

    if !APP_EXISTS.swap(true, Ordering::Relaxed) {
        QCoreApplication::new().into_raw();
    }
}
