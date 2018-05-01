// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use spec::{Target, TargetResult};

pub fn target() -> TargetResult {
    let mut base = super::l4re_base::opts();
    base.cpu = "x86-64".to_string();
    base.max_atomic_width = Some(64);

    Ok(Target {
        llvm_target: "x86_64-unknown-l4re-uclibc".to_string(),
        target_endian: "little".to_string(),
        target_pointer_width: "64".to_string(),
        target_c_int_width: "32".to_string(),
        data_layout: "e-m:e-i64:64-f80:128-n8:16:32:64-S128".to_string(),
        arch: "x86_64".to_string(),
        target_os: "l4re".to_string(),
        target_env: "uclibc".to_string(),
        target_vendor: "unknown".to_string(),
        options: base,
    })
}
