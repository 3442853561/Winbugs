// ffi.rs
extern crate libc;
use libc::c_int;
use libc::c_void;
use libc::size_t;

#[link(name = "winbugs")]
