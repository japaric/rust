// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// TODO(japaric) enforce feature gate
//#![feature(multiarg_indexing)]

use std::ops::Index;

struct Mat;

impl Index<(u32, u32)> for Mat {
    type Output = i32;

    fn index(&self, _: (u32, u32)) -> &i32 {
        static TWO: i32 = 2;

        &TWO
    }
}

struct Cube;

impl Index<(u32, u32, u32)> for Cube {
    type Output = i32;

    fn index(&self, _: (u32, u32, u32)) -> &i32 {
        static THREE: i32 = 3;

        &THREE
    }
}

fn main() {
    assert_eq!(Mat[1, 2], 2);
    assert_eq!(Cube[1, 2, 3], 3);
}
