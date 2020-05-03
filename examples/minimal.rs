use oculussdk::{*};
use std::os::raw::c_char;
use std::ffi::CStr;

unsafe extern fn callback(_: usize, l: i32, msg: *const c_char) {
    let msg = unsafe{
        CStr::from_ptr(msg)
    };
    println!("{:?}", msg);
}

fn main(){
    let session = Session::initialize(None, Some(callback)).unwrap();

}









