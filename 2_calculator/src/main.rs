use std::io;

fn get_number() -> f64{
    let guess: f64 = get_string().parse().expect("Not a number!");
    f64::from(guess)
}

fn get_string() -> String{
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    String::from(guess.trim())
}
//  how to ensure user input is Lower Case? '.make_ascii_lowercase();'?

fn main() {
    println!("Welcome to the Calculator. Please enter your name:");
    let username = get_string();
    println!("Hello {}!", username);
    println!("Okay {}, please enter your first number:", username);
    let input_1: f64 = get_number();
    println!("Thanks! {} - a good choice. Right, now your second number:", input_1);
    let input_2: f64 = get_number();
    println!("Great. So I have {} and {}. What operation would you like to proform {}?",input_1,input_2,username);
    let operation = get_string();
    let result: f64;
    let op;
    if operation == "times" || operation == "multiply" || operation == "*" || operation == "x" {
        result = input_1 * input_2;
        op = "x";
    } 
    else if operation == "devide" || operation == "/" {
        result = input_1 / input_2;
        op = "/";
    }
    else if operation == "subtract"|| operation == "minus" {
        result = input_1 - input_2;
        op = "-";
    }
    else if operation == "sum"|| operation == "plus"|| operation == "addition"|| operation == "add" {
        result = input_1 + input_2;
        op = "+";
    }
    else {
        result = 0.0;
        op = "?"
    }
    println!("{} {} {} = {}",input_1, op, input_2, result);
}
