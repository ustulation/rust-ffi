#![feature(libc)]

extern crate rustlib;
extern crate libc;

use std::ptr;
use std::mem;

use rustlib::TestTrait;

#[no_mangle]
pub extern fn free_function_wrapper(value: i32) -> i32 {
    rustlib::free_function(value)
}

#[no_mangle]
pub extern fn new_test_struct_wrapper(value: i32) -> *mut libc::c_void {
    let obj = rustlib::TestStruct::new(value);
    unsafe {
        let raw_ptr = libc::malloc(mem::size_of::<rustlib::TestStruct>() as libc::size_t) as *mut rustlib::TestStruct;

        // Don't do *raw_ptr = something as *raw_ptr is uninitialized and rust would attempt to
        // destroy the previous value (uninitialized here) held by *raw_ptr before putting in a new one which is
        // undefined. So overwrite it moving the new value into it.
        ptr::write(&mut *raw_ptr, obj);
        raw_ptr as *mut libc::c_void
    }
}

#[no_mangle]
pub extern fn test_struct_decrement_wrapper(raw_ptr: *mut libc::c_void, delta: i32) {
    unsafe {
        mem::transmute::<*mut libc::c_void, &mut rustlib::TestStruct>(raw_ptr).decrement(delta);
    }
}

#[no_mangle]
pub extern fn test_struct_trait_function_wrapper(raw_ptr: *mut libc::c_void) -> i32 {
    unsafe {
        mem::transmute::<*mut libc::c_void, &mut rustlib::TestStruct>(raw_ptr).trait_func()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn free_function_wrapper_test() {
        assert_eq!(free_function_wrapper(33), 11);
    }

    #[test]
    fn new_test_struct_wrapper_test() {
        let void_ptr = new_test_struct_wrapper(30);
        assert!(!void_ptr.is_null());
    }

    #[test]
    fn test_struct_trait_function_wrapper_test() {
        let void_ptr = new_test_struct_wrapper(32);
        assert!(!void_ptr.is_null());
        test_struct_decrement_wrapper(void_ptr, 12);
        assert_eq!(test_struct_trait_function_wrapper(void_ptr), 23);
    }
}
