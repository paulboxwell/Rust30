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


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let rect1 = Rectangle {width: 20, height: 20};
        assert_eq!(rect1.area(), 400);
    }
}