//! Utilities for manipulating fat pointers

use marker::{Unsized, Sized};
use raw::FatPtr;

extern "rust-intrinsic" {
    // FIXME(japaric) make `transmute` work with fat pointers
    fn unchecked_transmute<A, B>(_: A) -> B;
}

/// Creates a fat pointer from its "raw" representation
pub fn new<T: ?Sized + Unsized>(repr: FatPtr<T::Data, T::Info>) -> *mut T {
    unsafe {
        unchecked_transmute(repr)
    }
}

/// Returns the "raw" representation of the fat pointer `&T`
pub fn repr<T: ?Sized + Unsized>(fat_ptr: &T) -> FatPtr<T::Data, T::Info> {
    unsafe {
        unchecked_transmute(fat_ptr)
    }
}
