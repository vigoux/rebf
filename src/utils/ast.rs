use super::machine::Operation;
use std::fmt;
use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum AST {
    Instructions(Vec<Operation>, Box<AST>),
    Loop(Box<AST>, Box<AST>),
    EOF,
}

impl fmt::Display for AST {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Instructions(operations, next) => {
                for op in operations {
                    write!(f, "{}", op.value())?;
                }
                next.fmt(f)?;
            }
            Self::Loop(body, next) => {
                write!(f, "[")?;
                body.fmt(f)?;
                write!(f, "]")?;
                next.fmt(f)?;
            }
            Self::EOF => {}
        }
        Ok(())
    }
}

impl AST {
    fn box_if_not_empty(ops: Vec<Operation>, ast: AST) -> AST {
        if ops.len() != 0 {
            AST::Instructions(ops, Box::from(ast))
        } else {
            ast
        }
    }

    pub fn from(program: &mut Chars) -> AST {
        let mut operations_vec: Vec<Operation> = Vec::new();

        while let Some(instr) = program.next() {
            if let Some(operation) = Operation::from(instr) {
                operations_vec.push(operation);
            } else {
                match instr {
                    '[' => {
                        return AST::box_if_not_empty(
                            operations_vec,
                            AST::Loop(Box::from(AST::from(program)), Box::from(AST::from(program))),
                        )
                    }
                    ']' => break,
                    _ => {}
                }
            }
        }

        AST::box_if_not_empty(operations_vec, AST::EOF)
    }

    pub fn from_string(program: String) -> AST {
        AST::from(&mut program.chars())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty_ast() {
        let empty = AST::from_string(String::new());

        assert_eq!(empty, AST::EOF);
    }

    #[test]
    fn one_instruction() {
        let one = AST::from_string(String::from("."));

        assert_eq!(
            one,
            AST::Instructions(vec![Operation::Print], Box::from(AST::EOF))
        );
    }

    #[test]
    fn multiple_instructions() {
        let mult = AST::from_string(String::from(".#.#"));

        assert_eq!(
            mult,
            AST::Instructions(
                vec![
                    Operation::Print,
                    Operation::Debug,
                    Operation::Print,
                    Operation::Debug
                ],
                Box::from(AST::EOF)
            )
        )
    }

    #[test]
    fn empty_loop() {
        let bf_loop = AST::from_string(String::from("[]"));

        assert_eq!(bf_loop, AST::Loop(Box::from(AST::EOF), Box::from(AST::EOF)));
    }

    #[test]
    fn prefixed_loop() {
        let pref_loop = AST::from_string(String::from("..[]"));

        assert_eq!(
            pref_loop,
            AST::Instructions(
                vec![Operation::Print, Operation::Print],
                Box::from(AST::Loop(Box::from(AST::EOF), Box::from(AST::EOF)))
            )
        );
    }

    #[test]
    fn simple_loop() {
        let simple_loop = AST::from_string(String::from("[..]"));

        assert_eq!(
            simple_loop,
            AST::Loop(
                Box::from(AST::Instructions(
                    vec![Operation::Print, Operation::Print],
                    Box::from(AST::EOF)
                )),
                Box::from(AST::EOF)
            )
        );
    }

    #[test]
    fn normal_loop() {
        let bf_loop = AST::from_string(String::from("[.]."));

        assert_eq!(
            bf_loop,
            AST::Loop(
                Box::from(AST::Instructions(
                    vec![Operation::Print],
                    Box::from(AST::EOF)
                )),
                Box::from(AST::Instructions(
                    vec![Operation::Print],
                    Box::from(AST::EOF)
                ))
            )
        );
    }
}
