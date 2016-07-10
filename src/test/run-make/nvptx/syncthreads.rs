#![feature(lang_items)]
#![feature(no_core)]
#![feature(platform_intrinsics)]

#![no_core]

#![allow(dead_code)]

extern "platform-intrinsic" {
    fn nvptx_syncthreads();
}

fn syncthreads() {
    unsafe {
        nvptx_syncthreads();
    }
}

#[lang = "copy"]
trait Copy {}

#[lang = "sized"]
trait Sized {}
