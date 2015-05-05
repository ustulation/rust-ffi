pub fn free_function(value: i32) -> i32 {
    value / 3
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn free_function_test() {
        assert_eq!(free_function(27), 9);
    }
}
