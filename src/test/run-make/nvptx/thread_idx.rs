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
