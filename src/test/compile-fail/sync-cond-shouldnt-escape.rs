// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// error-pattern: reference is not valid outside of its lifetime
extern mod std;
use std::sync;

fn main() {
    let m = ~sync::Mutex();
    let mut cond = None;
    do m.lock_cond |c| {
        cond = Some(c);
    }   
    option::unwrap(cond).signal();
}
