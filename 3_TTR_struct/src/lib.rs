#[warn(dead_code)]

pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn area(&self) -> u32 {
        self.width * self.height
    }
}

/*
struct Example {
    number: i32,
}

impl Example {
    fn boo() {
        println!("boo! Example::boo() was called!");
    }

    fn answer(&mut self) {
        self.number += 42;
    }

    fn get_number(&self) -> i32 {
        self.number
    }
}

trait Thingy {
    fn do_thingy(&self);
}

impl Thingy for Example {
    fn do_thingy(&self) {
        println!("doing a thing! also, number is {}!", self.number);
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let val = get_number("1");
        assert_eq!(val, 1);
    }
    #[test]
    fn test_2() {
        let val = get_number("99");
        assert_eq!(val, 99);
    }
    #[test]
    fn test_3() {
        let val = get_number("0");
        assert_eq!(val, 0);
    }
}
*/
