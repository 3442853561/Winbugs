extern crate libc;
use libc::size_t;
use libc::c_int;
use std::ffi::CString;
use std::os::raw::c_char;

enum cWindow {}

#[link(name="winbugs")]
extern "C" {
    fn wbMsgbox(text: *const c_char, title: *const c_char, style: c_int) -> size_t;
    fn screenWidth() -> size_t;
    fn screenHeight() -> size_t;
    fn wbAbout(appName: *const c_char, appString: *const c_char);
    fn cWindow_new(name: *const c_char) -> cWindow;
}

pub const OKONLY: u8 = 0;
pub const OKCANCEL: u8 = 1;
pub const ABORTRETRYIGNORE: u8 = 2;
pub const YESNOCANCEL: u8 = 3;
pub const YESNO: u8 = 4;
pub const RETRYCANCEL: u8 = 5;
pub const CANCELRETRYCONTINUE: u8 = 6;
pub const CRITICALPLUS: u8 = 16;
pub const QUESTIONPLUS: u8 = 32;
pub const EXCLAMATIONPLUS: u8 = 48;
pub const INFORMATIONPLUS: u8 = 64;
pub const UNKNOWN: u8 = 0;
pub const OK: u8 = 1;
pub const CANCEL: u8 = 2;
pub const ABORT: u8 = 3;
pub const RETRY: u8 = 4;
pub const IGNORE: u8 = 5;
pub const YES: u8 = 6;
pub const NO: u8 = 7;

struct Form {
    caption: String,

    style: u8,
}

fn txt(text: &str) -> CString {
    CString::new(text).unwrap()
}

pub fn screen_width() -> u32 {
    unsafe { screenWidth() as u32 }
}

pub fn screen_height() -> u32 {
    unsafe { screenHeight() as u32 }
}

pub fn msgbox(text: &str, title: &str, style: u8) -> u8 {
    let self_title = txt(title);
    let self_text = txt(text);
    let self_style: c_int = style as c_int;
    unsafe {
        match wbMsgbox(self_text.as_ptr(), self_title.as_ptr(), self_style) {
            1 => OK,
            3 => ABORT,
            4 => RETRY,
            5 => IGNORE,
            6 => YES,
            7 => NO,
            _ => UNKNOWN,
        }
    }
}

pub fn form_new(name: &str) {
    let self_name = txt(name);
    unsafe {
        let foo: cWindow = cWindow_new(self_name.as_ptr());
    }
}

pub fn about(name: &str, text: &str) {
    let self_name = txt(name);
    let self_text = txt(text);
    unsafe {
        wbAbout(self_name.as_ptr(), self_text.as_ptr());
    }
}

fn main() {
    // let width: size_t = unsafe {
    // 	screenWidth()
    // };
    // let height: size_t = unsafe {
    // 	screenHeight()
    // };
    // let x=msgbox("my_text","my_title",ABORTRETRYIGNORE + INFORMATIONPLUS);
    // print!("{}",x);
    // about("my_app","my_text");
    form_new("text");
}
