struct Rectangle {
    width: u8,
    height: u8
}

impl Rectangle {
    fn is_square(&self) -> bool {
        return self.width == self.height;
    }
}

fn main() {
    
}

fn give_two() -> i32{
   return 2;
}

// cargo test to run tests
#[cfg(test)]
mod testing_module {
    #[test]
    #[should_panic] // should panic if the test should fail
    fn test_basic() {
        assert!(1 == 1); // OK

        panic!("Oh no!"); // lets the test fail no matter the circumstances
    }

    #[test]
    //#[ignore] //ignore if it takes a long time
    fn test_equals() {
        assert_eq!(super::give_two(), 1 + 1);
        assert_ne!(super::give_two(), 1 + 2);
    }

    #[test]
    #[should_panic]
    fn test_structs() {
        let r = super::Rectangle {
            width: 50,
            height: 25
        };
        assert!(r.is_square());
    }
}
