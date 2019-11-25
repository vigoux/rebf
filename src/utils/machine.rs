use super::ast::*;
use std::fmt;
use std::io::{self, prelude::*};
use std::ops::{Index, IndexMut};

#[derive(Debug, PartialEq)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
pub enum Computation {
    Add,
    Substract,
}

#[derive(Debug, PartialEq)]
pub enum Operation {
    Move(Direction),
    Change(Computation),
    Print,
    Read,
    Debug,
}

impl Operation {
    pub fn value(&self) -> char {
        match self {
            Self::Move(Direction::Right) => '>',
            Self::Move(Direction::Left) => '<',
            Self::Change(Computation::Add) => '+',
            Self::Change(Computation::Substract) => '-',
            Self::Print => '.',
            Self::Read => ',',
            Self::Debug => '#',
        }
    }

    pub fn from(instr: char) -> Option<Operation> {
        match instr {
            '>' => Some(Self::Move(Direction::Right)),
            '<' => Some(Self::Move(Direction::Left)),
            '+' => Some(Self::Change(Computation::Add)),
            '-' => Some(Self::Change(Computation::Substract)),
            '.' => Some(Self::Print),
            ',' => Some(Self::Read),
            '#' => Some(Self::Debug),
            _ => None,
        }
    }
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

pub struct MachineState {
    pointer: usize,
    memory: Vec<u8>,
}

impl Index<usize> for MachineState {
    type Output = u8;

    fn index(&self, index: usize) -> &u8 {
        &self.memory[index]
    }
}

impl IndexMut<usize> for MachineState {
    fn index_mut(&mut self, index: usize) -> &mut u8 {
        &mut self.memory[index]
    }
}

impl fmt::Display for MachineState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, value) in self.memory.iter().enumerate() {
            write!(
                f,
                " {:02X} {} ",
                value,
                if index == self.pointer { "<" } else { " " }
            )?;
            if index % 15 == 0 && index != 0 {
                write!(f, "\n")?;
            }
        }

        Ok(())
    }
}

impl MachineState {
    pub fn new() -> MachineState {
        MachineState {
            pointer: 0,
            memory: vec![0],
        }
    }

    fn get_current(&self) -> u8 {
        self[self.pointer]
    }

    fn pointer_move(&mut self, direction: &Direction) -> () {
        match direction {
            Direction::Left => {
                if self.pointer != 0 {
                    self.pointer -= 1;
                }
            }
            Direction::Right => {
                self.pointer += 1;
                if self.pointer == self.memory.len() {
                    self.memory.push(0u8);
                }
            }
        };
    }

    fn change(&mut self, operation: &Computation) -> () {
        let pointer = self.pointer;
        match operation {
            Computation::Add => {
                if self.get_current() == 255 {
                    self[pointer] = 0;
                } else {
                    self[pointer] += 1;
                }
            }
            Computation::Substract => {
                if self.get_current() == 0 {
                    self[pointer] = 255;
                } else {
                    self[pointer] -= 1;
                }
            }
        }
    }

    fn print(&self) -> () {
        print!("{}", self.get_current() as char)
    }

    fn read(&mut self) -> io::Result<()> {
        let mut input: [u8; 1] = [0];
        io::stdin().read(&mut input)?;

        let pointer = self.pointer;
        self[pointer] = input[0];

        Ok(())
    }

    fn apply(&mut self, instr: &Operation) -> io::Result<&Vec<u8>> {
        match instr {
            Operation::Move(dir) => {
                self.pointer_move(dir);
            }
            Operation::Change(op) => {
                self.change(op);
            }
            Operation::Print => {
                self.print();
            }
            Operation::Read => {
                self.read()?;
            }
            Operation::Debug => {
                println!("{}", self);
            }
        }
        Ok(&self.memory)
    }

    pub fn run(&mut self, instructions: &AST) -> io::Result<&Vec<u8>> {
        match instructions {
            AST::Instructions(operations, next) => {
                for op in operations {
                    self.apply(op)?;
                }
                self.run(next)
            }
            AST::Loop(body, next) => {
                while self.get_current() != 0 {
                    self.run(body)?;
                }
                self.run(next)
            }
            AST::EOF => Ok(&self.memory),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Computation;
    use super::Direction;
    use super::*;

    #[test]
    fn operation_valid() {
        assert_eq!(
            Operation::from(Operation::Debug.value()),
            Some(Operation::Debug)
        );

        assert_eq!(
            Operation::from(Operation::Print.value()),
            Some(Operation::Print)
        );

        assert_eq!(
            Operation::from(Operation::Read.value()),
            Some(Operation::Read)
        );

        assert_eq!(
            Operation::from(Operation::Change(Computation::Add).value()),
            Some(Operation::Change(Computation::Add))
        );

        assert_eq!(
            Operation::from(Operation::Change(Computation::Substract).value()),
            Some(Operation::Change(Computation::Substract))
        );

        assert_eq!(
            Operation::from(Operation::Move(Direction::Left).value()),
            Some(Operation::Move(Direction::Left))
        );

        assert_eq!(
            Operation::from(Operation::Move(Direction::Right).value()),
            Some(Operation::Move(Direction::Right))
        );
    }

    #[test]
    fn change_value() {
        let mut machine = MachineState::new();

        machine.change(&Computation::Add);

        assert_eq!(machine.get_current(), 1);

        machine.change(&Computation::Substract);

        assert_eq!(machine.get_current(), 0);
    }

    #[test]
    fn move_ptr() {
        let mut machine = MachineState::new();

        machine.pointer_move(&Direction::Right);
        machine.change(&Computation::Add);

        assert_eq!(machine[1], 1);

        machine.pointer_move(&Direction::Left);
        machine.change(&Computation::Add);

        assert_eq!(machine[0], 1);
    }

    #[test]
    fn change_overflow() {
        let mut machine = MachineState::new();

        machine.change(&Computation::Substract);

        assert_eq!(machine.get_current(), 255);

        machine.change(&Computation::Add);

        assert_eq!(machine.get_current(), 0);
    }

    #[test]
    fn pointer_move_overflow() {
        let mut machine = MachineState::new();

        machine.change(&Computation::Add);
        machine.pointer_move(&Direction::Left);
        assert_eq!(machine.get_current(), 1);
    }
}
