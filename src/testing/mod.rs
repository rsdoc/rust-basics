#![allow(dead_code)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn add() {
        assert_eq!(2 + 3, 5);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 300,
            height: 300,
        };

        let smaller = Rectangle {
            width: 150,
            height: 150,
        };

        assert_eq!(true, larger.can_hold(&smaller));
    }
}
