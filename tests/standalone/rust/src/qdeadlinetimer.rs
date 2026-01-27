use std::time::Duration;

use cxx_qt_io::TimerType;

#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-qt-io/qdeadlinetimer.h");
        type QDeadlineTimer = cxx_qt_io::QDeadlineTimer;
    }

    #[namespace = "ffi::qdeadlinetimer"]
    extern "Rust" {
        fn from_millis(millis: i32) -> QDeadlineTimer;
        fn from_nanos(nanos: i32) -> QDeadlineTimer;
    }
}

use ffi::QDeadlineTimer;

fn from_millis(millis: i32) -> QDeadlineTimer {
    QDeadlineTimer::new(Duration::from_millis(millis as _), TimerType::CoarseTimer)
}

fn from_nanos(nanos: i32) -> QDeadlineTimer {
    QDeadlineTimer::new(Duration::from_nanos(nanos as _), TimerType::CoarseTimer)
}
