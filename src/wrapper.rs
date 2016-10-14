// wrapper.rs
fn msgbox(text:String,title:String) {
	unsafe {
		wbMsgbox(text as *const c_char,title as *const c_char);
	}
}