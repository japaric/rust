//! Conversion traits

use marker::Sized;

/// TODO
pub trait As<T: ?Sized> {
    /// TODO
    fn cvt_as(&self) -> &T;
}

impl<T: ?Sized> As<T> for T {
    fn cvt_as(&self) -> &T {
        self
    }
}

impl<'a, T: ?Sized> As<T> for &'a T {
    fn cvt_as(&self) -> &T {
        *self
    }
}
