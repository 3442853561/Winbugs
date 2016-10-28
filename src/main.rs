extern crate libc;
use libc::size_t;
use libc::c_int;
use std::ffi::CString;
use std::os::raw::c_char;


#[link(name="winbugs")]
extern "C" {
	fn wbMsgbox(text:  *const c_char, title:  *const c_char ,style: c_int);
	fn screenWidth()->size_t;
	fn screenHeight()->size_t;

}

fn txt(text: &str)->CString{
	CString::new(text).unwrap()
}

fn msgbox(text: &str,title: &str,style: u8) {
	let self_title = txt(title);
    let self_text = txt(text);
	let self_style: c_int = style as c_int;
	unsafe {
		wbMsgbox(self_text.as_ptr(),self_title.as_ptr(),self_style);
	}
}

fn main() {
	let width: size_t = unsafe {
		screenWidth()
	};
	print!("{}",width);
	let height: size_t = unsafe {
		screenHeight()
	};
	print!("{}",height);
    msgbox("my_text","my_title",1);
}
