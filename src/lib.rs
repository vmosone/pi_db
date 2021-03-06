#![feature(fnbox)]
#![crate_type = "rlib"]
#![feature(custom_derive,asm,box_syntax,box_patterns)]
#![feature(pointer_methods)]
#![feature(core_intrinsics)]
#![feature(type_ascription)]
#![feature(i128)]
#![feature(nll)]
#[allow(dead_code,unused_variables,non_snake_case,unused_parens,unused_assignments,unused_unsafe,unused_imports)]

extern crate pi_lib;
extern crate fnv;

pub mod db;
pub mod mgr;
pub mod tabs;
pub mod memery_db;
pub mod util;
