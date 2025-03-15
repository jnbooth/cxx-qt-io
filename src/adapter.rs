#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]
use std::ffi::c_char;
use std::marker::PhantomData;
use std::pin::Pin;

use cxx_qt::{Downcast, Upcast};

use crate::QIODevice;
use std::io::{self, Read, Write};

pub trait QIOAdaptable: Upcast<QIODevice> {
    fn flush(device: Pin<&mut Self>) -> bool;

    #[cold]
    fn get_error_kind(&self) -> io::ErrorKind {
        io::ErrorKind::Other
    }
}

pub struct QIOAdapter<'a, T> {
    inner: Pin<&'a mut QIODevice>,
    _marker: PhantomData<T>,
}

impl<'a, T: QIOAdaptable> QIOAdapter<'a, T> {
    pub fn new(value: Pin<&'a mut T>) -> Self {
        Self {
            inner: value.upcast_pin(),
            _marker: PhantomData,
        }
    }

    #[cold]
    fn get_error(&self) -> io::Error {
        let error_kind = (*self.inner)
            .downcast::<T>()
            .expect("failed to downcast device")
            .get_error_kind();

        io::Error::new(error_kind, String::from(&self.inner.error_string()))
    }
}

impl<'a, T: QIOAdaptable> From<Pin<&'a mut T>> for QIOAdapter<'a, T> {
    fn from(value: Pin<&'a mut T>) -> Self {
        Self::new(value)
    }
}

impl<'a, T: QIOAdaptable> Read for QIOAdapter<'a, T> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let buf_ptr = buf.as_mut_ptr().cast::<c_char>();
        let result = unsafe { self.inner.as_mut().read_unsafe(buf_ptr, buf.len() as i64) };
        if let Ok(n) = usize::try_from(result) {
            return Ok(n);
        }
        Err(self.get_error())
    }
}

impl<'a, T: QIOAdaptable> Write for QIOAdapter<'a, T> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let buf_ptr = buf.as_ptr().cast::<c_char>();
        let result = unsafe { self.inner.as_mut().write_unsafe(buf_ptr, buf.len() as i64) };
        if let Ok(n) = usize::try_from(result) {
            return Ok(n);
        }
        Err(self.get_error())
    }

    fn flush(&mut self) -> io::Result<()> {
        let device = self
            .inner
            .as_mut()
            .downcast_pin()
            .expect("failed to downcast device");

        if T::flush(device) {
            Ok(())
        } else {
            Err(self.get_error())
        }
    }
}
