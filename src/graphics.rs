use gl::types::*;

pub fn initialize() {
    unsafe {
        gl::load_with(|s| {
            let c_str = CString::new(s).unwrap();
            gl::get_proc_address(c_str.as_ptr()) as *const _
        });
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
    }
}

pub fn render() {
    unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
}