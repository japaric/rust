// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Panic support for libcore
//!
//! The core library cannot define panicking, but it does *declare* panicking. This
//! means that the functions inside of libcore are allowed to panic, but to be
//! useful an upstream crate must define panicking for libcore to use. The current
//! interface for panicking is:
//!
//! ```
//! # use std::fmt;
//! fn panic_impl(fmt: fmt::Arguments, file_line_col: &(&'static str, u32, u32)) -> !
//! # { loop {} }
//! ```
//!
//! This definition allows for panicking with any general message, but it does not
//! allow for failing with a `Box<Any>` value. The reason for this is that libcore
//! is not allowed to allocate.
//!
//! This module contains a few other panicking functions, but these are just the
//! necessary lang items for the compiler. All panics are funneled through this
//! one function. Currently, the actual symbol is declared in the standard
//! library, but the location of this may change over time.

#![allow(dead_code, missing_docs)]
#![unstable(feature = "core_panic",
            reason = "internal details of the implementation of the `panic!` \
                      and related macros",
            issue = "0")]

use any::Any;
use fmt;

#[cold] #[inline(never)] // this is the slow path, always
#[cfg_attr(not(stage0), lang = "panic")]
pub fn panic(expr_file_line_col: &(&'static str, &'static str, u32, u32)) -> ! {
    // Use Arguments::new_v1 instead of format_args!("{}", expr) to potentially
    // reduce size overhead. The format_args! macro uses str's Display trait to
    // write expr, which calls Formatter::pad, which must accommodate string
    // truncation and padding (even though none is used here). Using
    // Arguments::new_v1 may allow the compiler to omit Formatter::pad from the
    // output binary, saving up to a few kilobytes.
    let (expr, file, line, col) = *expr_file_line_col;
    panic_fmt(fmt::Arguments::new_v1(&[expr], &[]), &(file, line, col))
}

// FIXME: remove when SNAP
#[cold] #[inline(never)]
#[cfg(stage0)]
#[lang = "panic"]
pub fn panic_old(expr_file_line: &(&'static str, &'static str, u32)) -> ! {
    let (expr, file, line) = *expr_file_line;
    let expr_file_line_col = (expr, file, line, 0);
    panic(&expr_file_line_col)
}

#[cold] #[inline(never)]
#[cfg_attr(not(stage0), lang = "panic_bounds_check")]
fn panic_bounds_check(file_line_col: &(&'static str, u32, u32),
                     index: usize, len: usize) -> ! {
    panic_fmt(format_args!("index out of bounds: the len is {} but the index is {}",
                           len, index), file_line_col)
}

// FIXME: remove when SNAP
#[cold] #[inline(never)]
#[cfg(stage0)]
#[lang = "panic_bounds_check"]
fn panic_bounds_check_old(file_line: &(&'static str, u32),
                     index: usize, len: usize) -> ! {
    let (file, line) = *file_line;
    panic_fmt(format_args!("index out of bounds: the len is {} but the index is {}",
                           len, index), &(file, line, 0))
}

#[cold] #[inline(never)]
pub fn panic_fmt(fmt: fmt::Arguments, file_line_col: &(&'static str, u32, u32)) -> ! {
    #[allow(improper_ctypes)]
    extern {
        #[lang = "panic_fmt"]
        #[unwind]
        fn panic_impl(fmt: fmt::Arguments, file: &'static str, line: u32, col: u32) -> !;
    }
    let (file, line, col) = *file_line_col;
    unsafe { panic_impl(fmt, file, line, col) }
}

/// A struct providing information about a panic.
///
/// `PanicInfo` structure is passed to a panic hook set by the [`set_hook`]
/// function.
///
/// [`set_hook`]: ../../std/panic/fn.set_hook.html
///
/// # Examples
///
/// ```should_panic
/// use std::panic;
///
/// panic::set_hook(Box::new(|panic_info| {
///     println!("panic occured: {:?}", panic_info.payload().downcast_ref::<&str>().unwrap());
/// }));
///
/// panic!("Normal panic");
/// ```
#[unstable(feature = "panic_impl", reason = "recently added", issue = "0")]
#[derive(Debug)]
pub struct PanicInfo<'a> {
    message: Option<fmt::Arguments<'a>>,
    payload: &'a (Any + Send),
    location: Location<'a>,
}

impl<'a> PanicInfo<'a> {
    #[unstable(feature = "panic_info_internals",
               reason = "implementation detail",
               issue = "0")]
    pub fn __new(payload: &'a (Any + Send),
                 message: Option<fmt::Arguments<'a>>,
                 file: &'static str,
                 line: u32,
                 col: u32) -> Self {
        PanicInfo {
            message,
            payload,
            location: Location { file, line, col },
        }
    }

    /// TODO
    #[unstable(feature = "panic_impl", reason = "recently added", issue = "0")]
    pub fn message(&self) -> Option<&fmt::Arguments> {
        self.message.as_ref()
    }

    /// Returns the payload associated with the panic.
    ///
    /// This will commonly, but not always, be a `&'static str` or [`String`].
    ///
    /// [`String`]: ../../std/string/struct.String.html
    ///
    /// # Examples
    ///
    /// ```should_panic
    /// use std::panic;
    ///
    /// panic::set_hook(Box::new(|panic_info| {
    ///     println!("panic occured: {:?}", panic_info.payload().downcast_ref::<&str>().unwrap());
    /// }));
    ///
    /// panic!("Normal panic");
    /// ```
    #[stable(feature = "panic_hooks", since = "1.10.0")]
    pub fn payload(&self) -> &(Any + Send) {
        self.payload
    }

    /// Returns information about the location from which the panic originated,
    /// if available.
    ///
    /// This method will currently always return [`Some`], but this may change
    /// in future versions.
    ///
    /// [`Some`]: ../../std/option/enum.Option.html#variant.Some
    ///
    /// # Examples
    ///
    /// ```should_panic
    /// use std::panic;
    ///
    /// panic::set_hook(Box::new(|panic_info| {
    ///     if let Some(location) = panic_info.location() {
    ///         println!("panic occured in file '{}' at line {}", location.file(), location.line());
    ///     } else {
    ///         println!("panic occured but can't get location information...");
    ///     }
    /// }));
    ///
    /// panic!("Normal panic");
    /// ```
    #[stable(feature = "panic_hooks", since = "1.10.0")]
    pub fn location(&self) -> Option<&Location> {
        Some(&self.location)
    }
}

/// A struct containing information about the location of a panic.
///
/// This structure is created by the [`location`] method of [`PanicInfo`].
///
/// [`location`]: ../../std/panic/struct.PanicInfo.html#method.location
/// [`PanicInfo`]: ../../std/panic/struct.PanicInfo.html
///
/// # Examples
///
/// ```should_panic
/// use std::panic;
///
/// panic::set_hook(Box::new(|panic_info| {
///     if let Some(location) = panic_info.location() {
///         println!("panic occured in file '{}' at line {}", location.file(), location.line());
///     } else {
///         println!("panic occured but can't get location information...");
///     }
/// }));
///
/// panic!("Normal panic");
/// ```
#[unstable(feature = "panic_impl", reason = "recently added", issue = "0")]
#[derive(Debug)]
pub struct Location<'a> {
    file: &'a str,
    line: u32,
    col: u32,
}

impl<'a> Location<'a> {
    /// Returns the name of the source file from which the panic originated.
    ///
    /// # Examples
    ///
    /// ```should_panic
    /// use std::panic;
    ///
    /// panic::set_hook(Box::new(|panic_info| {
    ///     if let Some(location) = panic_info.location() {
    ///         println!("panic occured in file '{}'", location.file());
    ///     } else {
    ///         println!("panic occured but can't get location information...");
    ///     }
    /// }));
    ///
    /// panic!("Normal panic");
    /// ```
    #[stable(feature = "panic_hooks", since = "1.10.0")]
    pub fn file(&self) -> &str {
        self.file
    }

    /// Returns the line number from which the panic originated.
    ///
    /// # Examples
    ///
    /// ```should_panic
    /// use std::panic;
    ///
    /// panic::set_hook(Box::new(|panic_info| {
    ///     if let Some(location) = panic_info.location() {
    ///         println!("panic occured at line {}", location.line());
    ///     } else {
    ///         println!("panic occured but can't get location information...");
    ///     }
    /// }));
    ///
    /// panic!("Normal panic");
    /// ```
    #[stable(feature = "panic_hooks", since = "1.10.0")]
    pub fn line(&self) -> u32 {
        self.line
    }

    /// Returns the column from which the panic originated.
    ///
    /// # Examples
    ///
    /// ```should_panic
    /// #![feature(panic_col)]
    /// use std::panic;
    ///
    /// panic::set_hook(Box::new(|panic_info| {
    ///     if let Some(location) = panic_info.location() {
    ///         println!("panic occured at column {}", location.column());
    ///     } else {
    ///         println!("panic occured but can't get location information...");
    ///     }
    /// }));
    ///
    /// panic!("Normal panic");
    /// ```
    #[unstable(feature = "panic_col", reason = "recently added", issue = "42939")]
    pub fn column(&self) -> u32 {
        self.col
    }
}
