use std::cmp::Ordering;
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::time::Duration;

use cxx::{ExternType, type_id};

use crate::TimerType;
use crate::util::MSecs;

#[cxx::bridge]
mod ffi {
    #[repr(i32)]
    #[namespace = "rust::cxxqtio1"]
    enum QDeadlineTimerForeverConstant {
        Forever,
    }

    extern "C++" {
        include!("cxx-qt-lib/qtypes.h");
        type qint64 = cxx_qt_lib::qint64;
    }

    #[namespace = "Qt"]
    extern "C++" {
        include!("cxx-qt-lib/qt.h");
        type TimerType = crate::TimerType;
    }

    #[namespace = "rust::cxxqtio1"]
    extern "C++" {
        include!("cxx-qt-io/qdeadlinetimer.h");
        type QDeadlineTimerForeverConstant;
    }

    unsafe extern "C++" {
        type QDeadlineTimer = super::QDeadlineTimer;

        /// Returns a `QDeadlineTimer` that is expired but is guaranteed to contain the current time. Objects created by this function can participate in the calculation of how long a timer is overdue, using the [`deadline`](QDeadlineTimer::deadline) function.
        #[Self = "QDeadlineTimer"]
        fn current(timer_type: TimerType) -> QDeadlineTimer;

        #[doc(hidden)]
        #[Self = "QDeadlineTimer"]
        #[rust_name = "add_nsecs"]
        fn addNSecs(timer: QDeadlineTimer, nsecs: qint64) -> QDeadlineTimer;

        #[doc(hidden)]
        #[rust_name = "deadline_qint64"]
        fn deadline(&self) -> qint64;

        #[doc(hidden)]
        #[rust_name = "deadline_n_secs_qint64"]
        fn deadlineNSecs(&self) -> qint64;

        /// Returns `true` if this `QDeadlineTimer` object has expired, `false` if there remains time left. For objects that have expired, [`remaining_time()`](QDeadlineTimer::remaining_time) will return 0 and [`deadline`](QDeadlineTimer::deadline) will return a time point in the past.
        ///
        /// `QDeadlineTimer` objects created with [`QDeadlineTimer::forever`] never expire and this function always returns `false` for them.
        #[rust_name = "has_expired"]
        fn hasExpired(&self) -> bool;

        /// Returns `true` if this `QDeadlineTimer` object never expires, `false` otherwise. For timers that never expire, [`remaining_time`](QDeadlineTimer::remaining_time) always returns -1 and [`deadline`](QDeadlineTimer::deadline) returns the maximum value.
        #[rust_name = "is_forever"]
        fn isForever(&self) -> bool;

        #[doc(hidden)]
        #[rust_name = "remaining_time_qint64"]
        fn remainingTime(&self) -> qint64;

        #[doc(hidden)]
        #[rust_name = "remaining_time_n_secs_qint64"]
        fn remainingTimeNSecs(&self) -> qint64;

        #[doc(hidden)]
        #[rust_name = "set_deadline_qint64"]
        fn setDeadline(&mut self, msecs: qint64, timer_type: TimerType);

        #[doc(hidden)]
        #[rust_name = "set_precise_deadline_qint64"]
        fn setPreciseDeadline(&mut self, secs: qint64, nsecs: qint64, timer_type: TimerType);

        #[doc(hidden)]
        #[rust_name = "set_precise_remaining_time_qint64"]
        fn setPreciseRemainingTime(&mut self, secs: qint64, nsecs: qint64, timer_type: TimerType);

        #[doc(hidden)]
        #[rust_name = "set_remaining_time_qint64"]
        fn setRemainingTime(&mut self, msecs: qint64, timer_type: TimerType);

        /// Changes the timer type for this object to `timer_type`.
        ///
        /// The behavior for each possible value of `timer_type` is operating-system dependent. [`TimerType::PreciseTimer`] will use the most precise timer that Qt can find, with resolution of 1 millisecond or better, whereas `QDeadlineTimer` will try to use a more coarse timer for [`TimerType::CoarseTimer`] and [`TimerType::VeryCoarseTimer`].
        #[rust_name = "set_timer_type"]
        fn setTimerType(&mut self, timer_type: TimerType);

        /// Returns the timer type is active for this object.
        #[rust_name = "timer_type"]
        fn timerType(&self) -> TimerType;
    }

    #[namespace = "rust::cxxqtio1"]
    unsafe extern "C++" {
        include!("cxx-qt-io/common.h");

        #[rust_name = "qdeadlinetimer_plus"]
        fn operatorPlus(a: QDeadlineTimer, b: i64) -> QDeadlineTimer;
        #[rust_name = "qdeadlinetimer_minus"]
        fn operatorMinus(a: QDeadlineTimer, b: i64) -> QDeadlineTimer;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qdeadlinetimer_init_default"]
        fn construct() -> QDeadlineTimer;

        #[rust_name = "qdeadlinetimer_init_forever"]
        fn construct(constant: QDeadlineTimerForeverConstant) -> QDeadlineTimer;

        #[rust_name = "qdeadlinetimer_eq"]
        fn operatorEq(a: &QDeadlineTimer, b: &QDeadlineTimer) -> bool;
        #[rust_name = "qdeadlinetimer_cmp"]
        fn operatorCmp(a: &QDeadlineTimer, b: &QDeadlineTimer) -> i8;
    }
}

use ffi::QDeadlineTimerForeverConstant;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum QDeadlineTimerError {
    IsForever,
    IsPast(Duration),
}

/// The `QDeadlineTimer` class marks a deadline in the future.
///
/// Qt Documentation: [QDeadlineTimer](https://doc.qt.io/qt-6/qdeadlinetimer.html#details)
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct QDeadlineTimer {
    t1: i64,
    t2: u32,
    timer_type: TimerType,
}

impl Default for QDeadlineTimer {
    /// Constructs an expired `QDeadlineTimer` object. For this object, [`remaining_time`](QDeadlineTimer::remaining_time) will return 0.
    ///
    /// For optimization purposes, this function will not attempt to obtain the current time and will use a value known to be in the past. Therefore, [`deadline`](QDeadlineTimer::deadline) may return an unexpected value and this object cannot be used in calculation of how long it is overdue. If that functionality is required, use [`QDeadlineTimer::current`].
    fn default() -> Self {
        ffi::qdeadlinetimer_init_default()
    }
}

impl PartialEq for QDeadlineTimer {
    fn eq(&self, other: &Self) -> bool {
        ffi::qdeadlinetimer_eq(self, other)
    }
}

impl Eq for QDeadlineTimer {}

impl PartialOrd for QDeadlineTimer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for QDeadlineTimer {
    fn cmp(&self, other: &Self) -> Ordering {
        ffi::qdeadlinetimer_cmp(self, other).cmp(&0)
    }
}

impl QDeadlineTimer {
    /// Constructs a new `QDeadlineTimer` of the specified `timer_type` that will expire after `duration`.
    ///
    /// The behavior for each possible value of `timer_type` is operating-system dependent. [`TimerType::PreciseTimer`] will use the most precise timer that Qt can find, with resolution of 1 millisecond or better, whereas `QDeadlineTimer` will try to use a more coarse timer for [`TimerType::CoarseTimer`] and [`TimerType::VeryCoarseTimer`].
    pub fn new(duration: Duration, timer_type: TimerType) -> Self {
        let mut this = Self {
            t1: 0,
            t2: 0,
            timer_type: TimerType { repr: 0 },
        };
        this.set_duration(duration, timer_type);
        this
    }

    /// Returns the absolute time point for the deadline stored in `QDeadlineTimer` object, calculated in milliseconds relative to the reference clock, the same as [`QElapsedTimer::msecs_since_reference`](https://doc.qt.io/qt-6/qelapsedtimer.html#msecsSinceReference). The value will be in the past if this `QDeadlineTimer` has expired.
    ///
    /// If this `QDeadlineTimer` never expires, this function returns [`i64::MAX`],
    ///
    /// **Note:** For a more Rust-friendly option, see [`QDeadlineTimer::duration_since`].
    pub fn deadline(&self) -> i64 {
        self.deadline_qint64().into()
    }

    /// Returns the absolute time point for the deadline stored in `QDeadlineTimer` object, calculated in nanoseconds relative to the reference clock, the same as [`QElapsedTimer::msecs_since_reference`](https://doc.qt.io/qt-6/qelapsedtimer.html#msecsSinceReference). The value will be in the past if this `QDeadlineTimer` has expired.
    ///
    /// If this `QDeadlineTimer` never expires or the number of nanoseconds until the deadline can't be accommodated in the return type, this function returns [`i64::MAX`].
    ///
    /// **Note:** For a more Rust-friendly option, see [`QDeadlineTimer::duration_since`].
    pub fn deadline_n_secs(&self) -> i64 {
        self.deadline_n_secs_qint64().into()
    }

    /// Returns the duration that has elapsed since an `earlier` timer. Fails if `earlier` expires after this timer or if either timer is [`forever`](QDeadlineTimer::forever).
    pub fn duration_since(&self, earlier: QDeadlineTimer) -> Result<Duration, QDeadlineTimerError> {
        const FOREVER: i64 = i64::MAX;

        #[inline]
        fn construct_duration(
            constructor: fn(u64) -> Duration,
            diff: i64,
        ) -> Result<Duration, QDeadlineTimerError> {
            if diff >= 0 {
                Ok(constructor(diff.cast_unsigned()))
            } else {
                Err(QDeadlineTimerError::IsPast(constructor(
                    (-diff).cast_unsigned(),
                )))
            }
        }

        let nsecs = self.deadline_n_secs();
        if nsecs != FOREVER {
            let earlier_nsecs = earlier.deadline_n_secs();
            if earlier_nsecs != FOREVER {
                return construct_duration(Duration::from_nanos, nsecs - earlier_nsecs);
            }
        }
        let msecs = self.deadline();
        if msecs == FOREVER {
            return Err(QDeadlineTimerError::IsForever);
        }
        let earlier_msecs = earlier.deadline();
        if earlier_msecs == FOREVER {
            return Err(QDeadlineTimerError::IsForever);
        }
        construct_duration(Duration::from_millis, msecs - earlier_msecs)
    }

    /// `QDeadlineTimer` objects created with this function never expire. For such objects, [`remaining_time`](QDeadlineTimer::remaining_time) will return -1, [`deadline`](QDeadlineTimer::deadline) will return the maximum value, and [`is_forever`](QDeadlineTimer::is_forever) will return `true`.
    pub fn forever() -> Self {
        ffi::qdeadlinetimer_init_forever(QDeadlineTimerForeverConstant::Forever)
    }

    /// Returns the remaining time in this `QDeadlineTimer` object in milliseconds. If the timer has already expired, this function will return 0 and it is not possible to obtain the amount of time overdue with this function (to do that, see [`deadline`](QDeadlineTimer::deadline)). If the timer was set to never expire, this function returns -1.
    ///
    /// **Note:** For a more Rust-friendly option, see [`QDeadlineTimer::remaining_time_as_duration`].
    pub fn remaining_time(&self) -> i64 {
        self.remaining_time_qint64().into()
    }

    /// Returns the remaining time in this `QDeadlineTimer` object as a `Duration`. If the timer has already expired, this function will return [`Duration::ZERO`] and it is not possible to obtain the amount of time overdue with this function. If the timer was set to never expire, this function returns `None`.
    pub fn remaining_time_as_duration(&self) -> Option<Duration> {
        let time = self.remaining_time();
        let msecs = u64::try_from(time).ok()?;
        Some(Duration::from_micros(msecs))
    }

    /// Returns the remaining time in this `QDeadlineTimer` object in nanoseconds. If the timer has already expired, this function will return zero and it is not possible to obtain the amount of time overdue with this function. If the timer was set to never expire, this function returns -1.
    ///
    /// **Note:** For a more Rust-friendly option, see [`QDeadlineTimer::remaining_time_as_duration`].
    pub fn remaining_time_n_secs(&self) -> i64 {
        self.remaining_time_n_secs_qint64().into()
    }

    /// Sets the deadline for this `QDeadlineTimer` object to be the `msecs` absolute time point, counted in milliseconds since the reference clock (the same as [`QElapsedTimer::msecs_since_reference`](https://doc.qt.io/qt-6/qelapsedtimer.html#msecsSinceReference)), and the timer type to `timer_type`. If the value is in the past, this `QDeadlineTimer` will be marked as expired.
    ///
    /// If msecs is [`i64::MAX`] or the deadline is beyond a representable point in the future, this `QDeadlineTimer` will be set to never expire.
    ///
    /// **Note:** For a more Rust-friendly option, see [`QDeadlineTimer::set_duration`].
    pub fn set_deadline(&mut self, msecs: i64, timer_type: TimerType) {
        self.set_deadline_qint64(msecs.into(), timer_type);
    }

    /// Sets the remaining time for the timer to `duration`, using the specified `timer_type`.
    pub fn set_duration(&mut self, duration: Duration, timer_type: TimerType) {
        self.set_precise_remaining_time(
            duration.as_secs().try_into().unwrap_or(i64::MAX),
            duration.subsec_nanos().into(),
            timer_type,
        );
    }

    /// Sets the timer to never expire.
    pub fn set_forever(&mut self) {
        self.set_remaining_time(-1, self.timer_type());
    }

    /// Sets the deadline for this `QDeadlineTimer` object to be `secs` seconds and `nsecs` nanoseconds since the reference clock epoch (the same as [`QElapsedTimer::msecs_since_reference`](https://doc.qt.io/qt-6/qelapsedtimer.html#msecsSinceReference)), and the timer type to `timer_type`. If the value is in the past, this `QDeadlineTimer` will be marked as expired.
    ///
    /// If `secs` or `nsecs` is [`i64::MAX`], this `QDeadlineTimer` will be set to never expire. If `nsecs` is more than 1 billion nanoseconds (1 second), then `secs` will be adjusted accordingly.
    ///
    /// **Note:** For a more Rust-friendly option, see [`QDeadlineTimer::set_duration`].
    pub fn set_precise_deadline(&mut self, secs: i64, nsecs: i64, timer_type: TimerType) {
        self.set_precise_deadline_qint64(secs.into(), nsecs.into(), timer_type);
    }

    /// Sets the remaining time for this `QDeadlineTimer` object to `secs` seconds plus `nsecs` nanoseconds from now, if `secs` has a positive value. If `secs` is negative, this `QDeadlineTimer` will be set it to never expire (this behavior does not apply to `nsecs`). If both parameters are 0, this `QDeadlineTimer` will be marked as expired.
    ///
    /// For optimization purposes, if both `secs` and `nsecs` are 0, this function may skip obtaining the current time and may instead use a value known to be in the past. If that happens, [`deadline`](QDeadlineTimer::deadline) may return an unexpected value and this object cannot be used in calculation of how long it is overdue. If that functionality is required, use [`QDeadlineTimer::current`] and add time to it.
    ///
    /// The timer type for this `QDeadlineTimer` object will be set to the specified `timer_type`.
    ///
    /// **Note:** Prior to Qt 6.6, the only condition that caused the timer to never expire was when `secs` was -1.
    ///
    /// **Note:** For a more Rust-friendly option, see [`QDeadlineTimer::set_duration`].
    pub fn set_precise_remaining_time(&mut self, secs: i64, nsecs: i64, timer_type: TimerType) {
        self.set_precise_remaining_time_qint64(secs.into(), nsecs.into(), timer_type);
    }

    /// Sets the remaining time for this `QDeadlineTimer` object to `msecs` milliseconds from now, if `msecs` has a positive value. If `msecs` is 0, this `QDeadlineTimer` object will be marked as expired, whereas a negative value will set it to never expire.
    ///
    /// For optimization purposes, if `msecs` is zero, this function may skip obtaining the current time and may instead use a value known to be in the past. If that happens, [`deadline`](QDeadlineTimer::deadline) may return an unexpected value and this object cannot be used in calculation of how long it is overdue. If that functionality is required, use [`QDeadlineTimer::current`] and add time to it.
    ///
    /// The timer type for this `QDeadlineTimer` object will be set to the specified `timer_type`.
    ///
    /// **Note:** Prior to Qt 6.6, the only value that caused the timer to never expire was -1.
    ///
    /// **Note:** For a more Rust-friendly option, see [`QDeadlineTimer::set_duration`].
    pub fn set_remaining_time(&mut self, msecs: i64, timer_type: TimerType) {
        self.set_remaining_time_qint64(msecs.into(), timer_type);
    }
}

impl Add<Duration> for QDeadlineTimer {
    type Output = Self;

    /// Returns a `QDeadlineTimer` whose deadline is `rhs` later than the deadline stored in `self`. If `self` is set to never expire, this function returns a `QDeadlineTimer` that does not expire either.
    fn add(self, rhs: Duration) -> Self::Output {
        if let Ok(nsecs) = i64::try_from(rhs.as_nanos()) {
            return Self::add_nsecs(self, nsecs.into());
        }
        let timer = ffi::qdeadlinetimer_plus(self, rhs.as_millis().try_into().unwrap_or(i64::MAX));
        let nsecs = rhs.subsec_nanos();
        if nsecs == 0 {
            timer
        } else {
            Self::add_nsecs(timer, i64::from(nsecs).into())
        }
    }
}

impl AddAssign<Duration> for QDeadlineTimer {
    /// Extends this `QDeadlineTimer` object by `rhs`. If `self` is set to never expire, this function does nothing.
    fn add_assign(&mut self, rhs: Duration) {
        *self = *self + rhs;
    }
}

impl Sub<Duration> for QDeadlineTimer {
    type Output = Self;

    /// Returns a `QDeadlineTimer` whose deadline is `rhs` earlier than the deadline stored in `self`. If `self` is set to never expire, this function returns a `QDeadlineTimer` that does not expire either.
    fn sub(self, rhs: Duration) -> Self::Output {
        if let Ok(nsecs) = i64::try_from(rhs.as_nanos()) {
            return Self::add_nsecs(self, (-nsecs).into());
        }
        let timer = ffi::qdeadlinetimer_minus(self, rhs.as_millis().try_into().unwrap_or(i64::MAX));
        let nsecs = rhs.subsec_nanos();
        if nsecs == 0 {
            timer
        } else {
            Self::add_nsecs(timer, (-i64::from(nsecs)).into())
        }
    }
}

impl SubAssign<Duration> for QDeadlineTimer {
    /// Shortens this `QDeadlineTimer` object by `rhs`. If `self` is set to never expire, this function does nothing.
    fn sub_assign(&mut self, rhs: Duration) {
        *self = *self - rhs;
    }
}

impl From<Duration> for QDeadlineTimer {
    /// Constructs a `QDeadlineTimer` from a duration, using [`TimerType::CoarseTimer`] as its timer type. To specify the timer type, use [`QDeadlineTimer::new`].
    fn from(value: Duration) -> Self {
        Self::new(value, TimerType::CoarseTimer)
    }
}

impl MSecs for QDeadlineTimer {
    fn msecs(self) -> i32 {
        i32::try_from(self.remaining_time()).unwrap_or(i32::MAX)
    }
}

// SAFETY: Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QDeadlineTimer {
    type Id = type_id!("QDeadlineTimer");
    type Kind = cxx::kind::Trivial;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Eq)]
    struct TimerProps {
        expired: bool,
        forever: bool,
        remaining: Option<bool>,
    }

    impl From<QDeadlineTimer> for TimerProps {
        fn from(value: QDeadlineTimer) -> Self {
            Self {
                expired: value.has_expired(),
                forever: value.is_forever(),
                remaining: match value.remaining_time_as_duration() {
                    None => None,
                    Some(Duration::ZERO) => Some(false),
                    _ => Some(true),
                },
            }
        }
    }

    #[test]
    fn new() {
        let timer = QDeadlineTimer::new(Duration::from_secs(120), TimerType::CoarseTimer);
        assert_eq!(
            TimerProps::from(timer),
            TimerProps {
                expired: false,
                forever: false,
                remaining: Some(true)
            }
        );
    }

    #[test]
    fn current() {
        let timer = QDeadlineTimer::current(TimerType::CoarseTimer);
        assert_eq!(
            TimerProps::from(timer),
            TimerProps {
                expired: true,
                forever: false,
                remaining: Some(false)
            }
        );
        assert!(timer.has_expired());
    }

    #[test]
    fn forever() {
        let timer = QDeadlineTimer::forever();
        assert_eq!(
            TimerProps::from(timer),
            TimerProps {
                expired: false,
                forever: true,
                remaining: None,
            }
        );
    }

    #[test]
    fn duration_since() {
        let timer1 = QDeadlineTimer::new(Duration::from_secs(120), TimerType::CoarseTimer);
        let timer2 = timer1 + Duration::from_nanos(30);
        assert_eq!(timer2.duration_since(timer1), Ok(Duration::from_nanos(30)));
    }

    #[test]
    fn set_forever() {
        let mut timer = QDeadlineTimer::new(Duration::from_secs(120), TimerType::CoarseTimer);
        timer.set_forever();
        assert!(timer.is_forever());
    }
}
