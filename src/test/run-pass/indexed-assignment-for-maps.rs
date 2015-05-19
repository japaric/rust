// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Test indexing on hash maps

#![feature(indexed_assignment)]

use std::collections::HashMap;

fn main() {
    let mut map: HashMap<i32, Vec<i32>> = HashMap::new();

    // insert
    map[0] = vec![];

    assert_eq!(map[&0], []);

    // mutating method
    map[&0].push(1);

    assert_eq!(map[&0], [1]);

    // mutable reference
    for x in &mut map[&0] {
        *x = 0;
    }

    assert_eq!(map[&0], [0]);
}
