// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(lang_items)]
#![feature(no_core)]
#![feature(platform_intrinsics)]

#![no_core]

#![allow(dead_code)]

extern "platform-intrinsic" {
    fn nvptx_thread_idx_x() -> i32;
}

fn thread_idx() -> i32 {
    unsafe {
        nvptx_thread_idx_x()
    }
}

#[lang = "copy"]
trait Copy {}

#[lang = "sized"]
trait Sized {}
