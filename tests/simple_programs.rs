use rebf::{MachineState, AST};
use std::fs;

#[test]
fn hello_world() {
    let mut machine = MachineState::new();

    let instructions = fs::read_to_string("tests/hello_world.bf").expect("File not found.");

    let ast = AST::from(&mut instructions.chars());

    let memory = machine.run(&ast).expect("An error occured");
    let checked: Vec<u8> = vec![0x00, 0x00, 0x48, 0x64, 0x57, 0x21, 0x0A];

    for i in 0..checked.len() {
        assert_eq!(memory[i], checked[i]);
    }
}
