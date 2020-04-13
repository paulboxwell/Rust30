use mylib::Stack;

fn main() {
    println!("Stack");

    let mut mystack = Stack { vector: vec![1, 2] };

    println!(
        "The size of the stack is {}.",
        mystack.size()
    );
    mystack.push(0);
    println!(
        "The new size of the stack is {}.",
        mystack.size()
    );
}