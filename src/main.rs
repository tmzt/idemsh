
#[macro_use]
pub extern crate nom;

mod ast;
mod parser;
mod traits;
mod errors;
mod local_exec;
mod handle_exec;

fn main() {
    println!("Hello, world!");
}
