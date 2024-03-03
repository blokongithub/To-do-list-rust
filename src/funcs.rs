use std::io::{self, stdin, stdout, Bytes, Write};

pub fn getinput() -> String{

    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("error gtting input");
    input = input.trim().to_string();

    return input;
    
}



pub fn opiton() -> i8 {
    io::stdout().flush().expect("error flushing");
    println!("-------------------------");
    println!("1) view your todo list");
    println!("2) view your done list");
    println!("3) add to your todo list");
    println!("4) make a todo as done");
    println!("-------------------------");
    print!("Enter an option: ");
    io::stdout().flush().expect("error flushing");
    let mut input: i8 = getinput().trim().parse().unwrap();
    return input;
}