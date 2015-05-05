extern crate rustlib;

#[no_mangle]
pub extern fn free_function_wrapper(value: i32) -> i32 {
    rustlib::free_function(value)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn free_function_wrapper_test() {
        assert_eq!(free_function_wrapper(33), 11);
    }
}
