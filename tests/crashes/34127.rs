//@ compile-flags: -g -Copt-level=0
//@ known-bug: #34127

pub fn main() {
let _a = [(); 1 << 63];
}
