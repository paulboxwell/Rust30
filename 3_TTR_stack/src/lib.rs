#[warn(dead_code)]

pub struct Stack {
    pub vector: Vec<u32>,
}

impl Stack {
    pub fn push(&mut self, val: u32) {
        self.vector.push(val)
    }
    pub fn pop(&mut self) -> u32 {
        self.vector.pop().unwrap()
    }
    pub fn size(&self) -> usize {
        self.vector.len()
    }
    pub fn is_empty(&self) -> bool {
        self.vector.is_empty()
    }

    //set
    //get
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_return_size() {
        let mut uut = Stack { vector: vec![1, 2] };
        assert_eq!(uut.size(), 2); // confirm current size of stack is 2
    }
    #[test]
    fn can_pop() {
        let mut uut = Stack { vector: vec![1, 2] };
        assert_eq!(uut.size(), 2); // confirm current size of stack is 2
        let pop_out = uut.pop(); // pop item off stack
        assert_eq!(uut.size(), 1); // confrim size has reduced by 1.
        assert_eq!(pop_out, 2); // confrim poped item was caught
    }
    #[test]
    fn can_push() {
        let mut uut = Stack { vector: vec![1, 2] };
        assert_eq!(uut.size(), 2);
        uut.push(0);
        assert_eq!(uut.size(), 3);
        let pop_out = uut.pop(); // pop item off stack
        assert_eq!(pop_out, 0); // confrim poped item was caught
    }
    #[test]
    fn can_return_empty() {
        let mut uut = Stack { vector: vec![1] };
        assert_eq!(uut.is_empty(), false);
        uut.pop();
        assert_eq!(uut.is_empty(), true);
        assert_eq!(uut.size(), 0);
    }
}