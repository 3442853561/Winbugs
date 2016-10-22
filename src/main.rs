extern crate libc;
use libc::size_t;

#[link(name="winbugs")]
extern "C" {
    fn wbMsgbox(text: *const std::os::raw::c_uchar, title: *const ::std::os::raw::c_uchar);
	fn screenWidth()->size_t;
	fn screenHeight()->size_t;
}

fn main() {
    let title = "my_title".to_string();
    let text = "my_text".to_string();
    unsafe {
        wbMsgbox(title.as_ptr(),text.as_ptr());
    }
	let a: size_t = unsafe {
		screenWidth()
	};
	print!("{}",a);
}