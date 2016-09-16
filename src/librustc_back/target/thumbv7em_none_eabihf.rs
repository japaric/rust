// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use target::{Target, TargetOptions, TargetResult};

pub fn target() -> TargetResult {
    let base = super::thumb_base::opts();
    Ok(Target {
        llvm_target: "thumbv7em-none-eabihf".to_string(),
        target_endian: "little".to_string(),
        target_pointer_width: "32".to_string(),
        data_layout: "e-m:e-p:32:32-i64:64-v128:64:128-a:0:32-n32-S64".to_string(),
        arch: "arm".to_string(),
        target_os: "none".to_string(),
        target_env: "".to_string(),
        target_vendor: "".to_string(),

        options: TargetOptions {
            // NOTE lowest common denominator between the Cortex-M4 (vfp4) and the Cortex-M7 (vfp5)
            features: "+vfp4".to_string(),
            .. base
        }
    })
}
