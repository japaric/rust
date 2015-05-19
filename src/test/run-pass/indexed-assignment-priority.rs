// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Test that if `A: IndexMut<B, Output=C>` and `A: IndexAssign<B, C>`, then the expression
// `a[b] = c` will be evaluated using the `IndexAssign` trait. In other words, test that the
// `IndexAssign` has higher priority than the `IndexMut` trait.

#![feature(core)]
#![feature(indexed_assignment)]

use std::ops::{Index, IndexAssign, IndexMut};

struct Foo(());

impl Index<()> for Foo {
    type Output = ();

    fn index(&self, _: ()) -> &() {
        &self.0
    }
}

impl IndexMut<()> for Foo {
    fn index_mut(&mut self, _: ()) -> &mut () {
        unreachable!()
    }
}

impl IndexAssign<(), ()> for Foo {
    fn index_assign(&mut self, _: (), _: ()) {
        println!("OK");
    }
}

fn main() {
    let mut foo = Foo(());
    foo[()] = ();
}
