pub fn free_function(value: i32) -> i32 {
    value / 3
}

pub trait TestTrait {
    fn trait_func(&mut self) -> i32;
}

pub struct TestStruct {
    value: i32,
}

impl TestStruct {
    pub fn new(value: i32) -> TestStruct {
        TestStruct {
            value: value,
        }
    }

    pub fn decrement(&mut self, delta: i32) {
        self.value -= delta;
    }
}

impl TestTrait for TestStruct {
    fn trait_func(&mut self) -> i32 {
        self.value += 3;
        self.value
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn free_function_test() {
        assert_eq!(free_function(27), 9);
    }

    #[test]
    fn test_struct_test() {
        let mut obj0 = TestStruct::new(10);
        obj0.decrement(7);
        assert_eq!(obj0.trait_func(), 6);
    }
}
