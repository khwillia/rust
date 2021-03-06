// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

struct cat<U> {
  priv mut meows : uint,

  how_hungry : int,
}

impl<U> cat<U> {
  fn speak() {
    self.meows += 1u;
  }
  fn meow_count() -> uint { self.meows }
}

fn cat<U>(in_x : uint, in_y : int) -> cat<U> {
    cat {
        meows: in_x,
        how_hungry: in_y
    }
}


fn main() {
  let _nyan : cat<int> = cat::<int>(52u, 99);
  //  let kitty = cat(1000u, 2);
}
