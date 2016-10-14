// ffi.rs
extern crate libc;
use std::ffi::CString;
use std::os::raw::c_char;

#[link(name = "winbugs")]
extern {
	fn wbMsgbox(text: *const c_char,title: *const c_char);
}

fn main(){
	unsafe{
		let x=CString::new("test").unwrap();
		wbMsgbox(x.as_ptr(),x.as_ptr());
	}
}