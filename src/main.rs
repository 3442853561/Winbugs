extern crate libc;
use libc::size_t;
use libc::c_int;
use std::ffi::CString;
use std::os::raw::c_char;


#[link(name="winbugs")]
extern "C" {
	fn wbMsgbox(text: *const c_char, title: *const c_char ,style: c_int)->size_t;
	fn screenWidth()->size_t;
	fn screenHeight()->size_t;
	fn wbAbout(appName: *const c_char,appString: *const c_char);

}

const OKONLY: u8 = 0;
const OKCANCEL: u8 = 1;
const ABORTRETRYIGNORE: u8 = 2;
const YESNOCANCEL: u8 = 3;
const YESNO: u8 = 4;
const RETRYCANCEL: u8 = 5;
const CANCELRETRYCONTINUE: u8 = 6;
const CRITICALPLUS: u8 = 16;
const QUESTIONPLUS: u8 = 32;
const EXCLAMATIONPLUS: u8 = 48;
const INFORMATIONPLUS: u8 = 64;
const UNKNOWN: u8 = 0;
const OK:u8 = 1;
const CANCEL:u8 = 2;
const ABORT:u8 = 3;
const RETRY:u8 = 4;
const IGNORE:u8 = 5;
const YES:u8 = 6;
const NO:u8 = 7;

fn txt(text: &str)->CString{
	CString::new(text).unwrap()
}

fn msgbox(text: &str,title: &str,style: u8)->u8 {
	let self_title = txt(title);
    let self_text = txt(text);
	let self_style: c_int = style as c_int;
	unsafe {
		match wbMsgbox(self_text.as_ptr(),self_title.as_ptr(),self_style){
			1 => OK,
			2 => CANCEL,
			3 => ABORT,
			4 => RETRY,
			5 => IGNORE,
			6 => YES,
			7 => NO,
			_ => UNKNOWN,
		}
	}
}

fn about(name: &str,text: &str) {
	let self_name = txt(name);
    let self_text = txt(text);
	unsafe {
		wbAbout(self_name.as_ptr(),self_text.as_ptr());
	}
}

fn main() {
	let width: size_t = unsafe {
		screenWidth()
	};
	let height: size_t = unsafe {
		screenHeight()
	};
    let x=msgbox("my_text","my_title",ABORTRETRYIGNORE + INFORMATIONPLUS);
	print!("{}",x);
	about("my_app","my_text");
}
