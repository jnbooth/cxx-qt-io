#[cxx::bridge(namespace = "Qt")]
mod ffi {
    #[repr(i32)]
    #[derive(Debug, PartialEq, Eq)]
    enum TimerType {
        /// Precise timers try to keep millisecond accuracy
        PreciseTimer,
        /// Coarse timers try to keep accuracy within 5% of the desired interval
        CoarseTimer,
        /// Very coarse timers only keep full second accuracy
        VeryCoarseTimer,
    }

    unsafe extern "C++" {
        include!("cxx-qt-lib/qt.h");
        type TimerType;
    }
}

pub use ffi::TimerType;
