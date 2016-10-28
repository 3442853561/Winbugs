extern crate libc;
use libc::size_t;
use libc::c_int;
use std::ffi::CString;
use std::os::raw::c_char;


#[link(name="winbugs")]
extern "C" {
	fn wbMsgbox(text:  *const c_char, title:  *const c_char ,style: c_int)->size_t;
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
const RS_UNKNOWN: u8 = 0;
const RS_OK:u8 = 1;
const RS_CANCEL:u8 = 2;
const RS_ABORT:u8 = 3;
const RS_RETRY:u8 = 4;
const RS_IGNORE:u8 = 5;
const RS_YES:u8 = 6;
const RS_NO:u8 = 7;

fn txt(text: &str)->CString{
	CString::new(text).unwrap()
}

fn msgbox(text: &str,title: &str,style: u8)->u8 {
	let self_title = txt(title);
    let self_text = txt(text);
	let self_style: c_int = style as c_int;
	unsafe {
		match wbMsgbox(self_text.as_ptr(),self_title.as_ptr(),self_style){
			1 => RS_OK,
			2 => RS_CANCEL,
			3 => RS_ABORT,
			4 => RS_RETRY,
			5 => RS_IGNORE,
			6 => RS_YES,
			7 => RS_NO,
			_ => RS_UNKNOWN,
		}
	}
}

fn main() {
	let width: size_t = unsafe {
		screenWidth()
	};
	let height: size_t = unsafe {
		screenHeight()
	};
    let x=msgbox("my_text","my_title",RS_ABORTRETRYIGNORE + RS_INFORMATIONPLUS);
	print!("{}",x);
}
