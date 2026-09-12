pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod adder_tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn can_first_hold_second_rect() {
        let rect1 = Rectangle {
            width: 65,
            height: 32,
        };
        let other = Rectangle {
            width: 43,
            height: 31,
        };
        assert!(rect1.can_hold(&other))
    }

    #[test]
    #[should_panic(expected = "big")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn greater_than_100_results() -> Result<(), String> {
        let num = Guess::large(100);
        if num.value > 100 {
            Ok(())
        } else {
            Err(String::from("Number less than 100"))
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("too small");
        } else if value > 100 {
            panic!("too big");
        }
        Guess { value }
    }
    pub fn large(value: i32) -> Guess {
        Guess { value }
    }
}
