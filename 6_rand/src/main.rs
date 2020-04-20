extern crate rand;
use rand::prelude::*;
use std::io;

fn get_number() -> u32{
    let guess: u32 = get_string().parse().expect("Not a number!");
    u32::from(guess)
}

fn get_string() -> String{
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    String::from(guess.trim())
}

fn draw_tree(size: u32) {
    println!("float: {}", size);
    for x in 1..size {
        for i in 0..x {
            print!("*");
        }
        println!("");
    }


}

fn main() {
    println!("Welcome to the Christmas tree maker. Please enter your name:");
    let username = get_string();
    println!("Hello {}!", username);
    loop
    {
        println!("Okay {}, How big would you like your christmas tree?", username);

        let input_1: u32 = get_number();
        draw_tree(input_1);

        println!("again? y/n");
        let again = get_string();
        if again == "n"
        {
            break;
        }
        
    }
    
}
/*
fn main() {
    if rand::random() { // generates a boolean
        // Try printing a random unicode code point (probably a bad idea)!
        println!("char: {}", rand::random::<char>());
    }
    
    let mut rng = rand::thread_rng();
    let _y: f64 = rng.gen(); // generates a float between 0 and 1
    println!("float: {}", _y);



    let mut nums: Vec<i32> = (1..100).collect();

    for x in 0..4 {
        println!("{}", x); // x: i32
        println!("{}", nums[x]);
    }

    nums.shuffle(&mut rng);


    for x in 4..0 {
        println!("{}", x); // x: i32
        println!("{}", nums[x]);
    }


}
*/