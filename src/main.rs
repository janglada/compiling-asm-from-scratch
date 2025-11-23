extern crate core;

// mod arm;
mod ast;

mod arm_code_generator;
mod emitter;
mod error;
mod parser;
mod visitor;

fn main() {
    println!("Hello, world!");
}
