#[warn(dead_code)]
pub fn get_number(msg: &str) -> u32{
    let guess: u32 = msg.parse().expect("Not a number!");
    u32::from(guess)
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