// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/*!
 * Try to double-check that static fns have the right size (with or
 * without dummy env ptr, as appropriate) by iterating a size-2 array.
 * If the static size differs from the runtime size, the second element
 * should be read as a null or otherwise wrong pointer and crash.
 */

fn f() { }
static bare_fns: &'static [fn()] = &[f, f];
struct S<'a>(||:'a);
static mut closures: &'static mut [S<'static>] = &mut [S(f), S(f)];

pub fn main() {
    unsafe {
        for &bare_fn in bare_fns.iter() { bare_fn() }
        for closure in closures.mut_iter() {
            let S(ref mut closure) = *closure;
            (*closure)()
        }
    }
}
