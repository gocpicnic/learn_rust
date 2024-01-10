#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
struct Point {
    x: i32,
    y: i32,
}
#[automatically_derived]
impl ::core::fmt::Debug for Point {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(f, "Point", "x",
            &self.x, "y", &&self.y)
    }
}

fn main() {
    let origin: Point = Point { x: 0, y: 0 };
    // let minmax = MinMax(0, 14);
    { ::std::io::_print(format_args!("Hello, world!\n")); };
    { ::std::io::_print(format_args!("Display: {0:?}\n", origin)); };
    match (&{
                    let res =
                        ::alloc::fmt::format(format_args!("The origin is: {0:?}",
                                origin));
                    res
                }, &"The origin is: Point { x: 0, y: 0 }") {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(kind, &*left_val,
                        &*right_val, ::core::option::Option::None);
                }
        }
    };
}
