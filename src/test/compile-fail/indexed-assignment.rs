// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(core)]
#![feature(indexed_assignment)]

use std::collections::HashMap;
use std::mem;
use std::ops::{Index, IndexAssign, Range, RangeFull};

struct Array;
struct Foo;

impl IndexAssign<Foo, Foo> for Array {
    fn index_assign(&mut self, _: Foo, _: Foo) {
        unimplemented!()
    }
}

fn lhs_not_mutable() {
    let ref array = Array;
    array[Foo] = Foo;  //~ error: cannot borrow immutable borrowed content `*array` as mutable

    let array = Array;
    array[Foo] = Foo;  //~ error: cannot borrow immutable local variable `array` as mutable
}

fn double_move() {
    let foo = Foo;
    let mut array = Array;

    array[
        foo   //~ error: use of moved value: `foo`
    ] = foo;  //~ note: `foo` moved here
}

fn main() {}
