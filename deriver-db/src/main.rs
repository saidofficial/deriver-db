//lints
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::io;
use colored::Colorize;

pub mod login;
mod logstate;
mod admin;


//main
fn main() {
    println!("{}", format!("deriver terminal ->").cyan());
    print!("{}", format!(">>").green());
    let mut userInput = String::new();
    io::stdin().read_line(&mut userInput);

}