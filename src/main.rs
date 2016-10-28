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

const RS_OKONLY: u8 = 0;
const RS_OKCANCEL: u8 = 1;
const RS_ABORTRETRYIGNORE: u8 = 2;
const RS_YESNOCANCEL: u8 = 3;
const RS_YESNO: u8 = 4;
const RS_RETRYCANCEL: u8 = 5;
const RS_CANCELRETRYCONTINUE: u8 = 6;
const RS_CRITICALPLUS: u8 = 16;
const RS_QUESTIONPLUS: u8 = 32;
const RS_EXCLAMATIONPLUS: u8 = 48;
const RS_INFORMATIONPLUS: u8 = 64;

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
    msgbox("my_text","my_title",RS_OKONLY + RS_INFORMATIONPLUS);
}
