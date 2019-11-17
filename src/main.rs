use rebf::{MachineState, AST};
use std::fs;
use std::env;

fn main() {
    let args : Vec<String> = env::args().collect();

    if args.len() >= 2 {
        let mut machine = MachineState::new();

        let instructions = fs::read_to_string(args[1].as_ref() as &str).expect("File not found.");

        let ast = AST::from(&mut instructions.chars());
        
        machine.run(&ast).expect("Execution failed");
    } else {
        println!("Usage : {} [SOURCE_FILE]", args[0]);
    }
}
