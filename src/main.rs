extern crate libc;
use libc::size_t;
use std::ffi::CString;
use std::os::raw::c_char;

#[link(name="winbugs")]
extern "C" {
    fn wbMsgbox(text:  *const c_char, title:  *const c_char);
	fn screenWidth()->size_t;
	fn screenHeight()->size_t;

}

fn txt(text: &str)->CString{
	CString::new(text).unwrap()
}

fn main() {
    let title = txt("my_title");
    let text = txt("my_text");
	let width: size_t = unsafe {
		screenWidth()
	};
	print!("{}",width);
	let height: size_t = unsafe {
		screenHeight()
	};
	print!("{}",height);
	unsafe {
        wbMsgbox(text.as_ptr(),title.as_ptr());
    }
}