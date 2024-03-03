use std::{io::{self, Write}, iter::Enumerate};
mod funcs;

fn select_op(choice: i8, mut todo: &mut Vec<String>, mut done: &mut Vec<String>) {
    match choice {
        1 => viewlist(&mut todo),
        2 => viewlist(&mut done),
        3 => addlist(&mut todo),
        4 => swap_lists(&mut todo, &mut done),
        _ => println!("Invalid option")
    };
}

fn viewlist(data: &mut Vec<String>) {
    println!("-------------------------");
    for (index, item) in data.iter().enumerate() {
        println!("{}) {}", index, item)
    }
    println!("-------------------------");
}

fn addlist(data: &mut Vec<String>) {

    print!("Enter a string: ");

    io::stdout().flush().expect("error flushing");

    let input: String = funcs::getinput();
    data.push(input);
}

fn swap_lists(from_ls: &mut Vec<String>, to_ls: &mut Vec<String>) {
    viewlist(from_ls);
    
    print!("What index would you like to change over: ");
    io::stdout().flush().expect("error flushing");
    let mut input: usize = funcs::getinput().trim().parse().unwrap();

    let mut change: String = String::new();
    let change: String = from_ls[input].to_string();
    
    to_ls.push(change);
    from_ls.remove(input);
}

fn main() {
    let mut todo: Vec<String> = Vec::new();
    let mut done: Vec<String> = Vec::new();

    let mut choice: i8 = funcs::opiton();
    loop {
        println!(" ");
        select_op(choice, &mut todo, &mut done);
        choice = funcs::opiton();
    }
}