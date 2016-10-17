#[link(name="winbugs")]

extern "C" {
    fn wbMsgbox(text: *const std::os::raw::c_uchar, title: *const ::std::os::raw::c_uchar);
}

fn main() {
    let title = "my_title".to_string();
    let text = "my_text".to_string();
    unsafe {
        wbMsgbox(title.as_ptr(),text.as_ptr());
    }
}