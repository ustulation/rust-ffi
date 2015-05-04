#![crate_type = "staticlib"]

#[no_mangle]
pub extern fn rust_function(input: i32) -> i32 {
    input * 2
}
