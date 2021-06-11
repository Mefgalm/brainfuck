use crate::parser::*;

//use std::io;
//use std::fmt;

pub struct Program {
    cells: [u8; 30_000],
    pointer: usize,
}

impl Program {
    pub fn new() -> Program {
        Program {
            cells: [0; 30_000],
            pointer: 0,
        }
    } 

    fn inc_pointer(&mut self) {
        self.pointer += 1;
    }

    fn dec_pointer(&mut self) {
        self.pointer -= 1;
    }

    fn inc_data(&mut self) {
        self.cells[self.pointer] += 1;
    }

    fn dec_data(&mut self) {
        self.cells[self.pointer] -= 1;
    }

    fn output(&self) {
        print!("{}", self.cells[self.pointer] as char);
    }

    fn accept(&mut self) {
    }

    fn pointer_cell(&self) -> u8 {
        self.cells[self.pointer]
    }
}

pub fn run(expressions: &Vec<Expression>, program: &mut Program) {
    for e in expressions {
        match e {
            Expression::IncPointer => program.inc_pointer(),
            Expression::DecPointer => program.dec_pointer(),
            Expression::IncData => program.inc_data(),
            Expression::DecData => program.dec_data(),
            Expression::Output => program.output(),
            Expression::Accept => program.accept(),
            Expression::Loop(loop_expressions) => {
                while program.pointer_cell() != 0 {
                    run(&loop_expressions, program)
                }
            }
        }
    }
}
