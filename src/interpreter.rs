use std::io::{Read, Write};

use crate::op::Op;

const CELL_SIZE: usize = 30_000;

#[derive(Debug)]
pub struct Interpreter {
    cells: [u8; CELL_SIZE],
    cursor: usize,
    pc: usize,
}

impl Default for Interpreter {
    fn default() -> Self {
        Interpreter {
            cells: [0; CELL_SIZE],
            pc: 0,
            cursor: 0,
        }
    }
}

impl Interpreter {
    pub fn interpret(ops: &[Op]) {
        Interpreter::default().run_ops(ops)
    }

    pub fn increment_pc(&mut self) {
        self.pc += 1;
    }

    fn run_op(&mut self, op: Op) {
        match op {
            Op::Left(n) => {
                self.cursor -= n;
            }
            Op::Right(n) => self.cursor += n,
            Op::Plus(n) => {
                let curr_val = self.cells[self.cursor];
                self.cells[self.cursor] = curr_val.wrapping_add(n as u8);
            }
            Op::Minus(n) => {
                let curr_val = self.cells[self.cursor];
                self.cells[self.cursor] = curr_val.wrapping_sub(n as u8);
            }
            Op::GetChar => {
                let mut buf: [u8; 1] = [0];
                std::io::stdout().flush().unwrap();

                let mut stdin = std::io::stdin();
                stdin.read_exact(&mut buf).unwrap();
                self.cells[self.cursor] = buf[0];
            }
            Op::PutChar => {
                let chary = self.cells[self.cursor] as char;
                print!("{chary}");
            }
            Op::LoopStart(addr) => {
                let curr_cell = self.cells[self.cursor];
                if curr_cell == 0 {
                    self.pc = addr;
                }
            }
            Op::LoopEnd(addr) => {
                let curr_cell = self.cells[self.cursor];
                if curr_cell != 0 {
                    self.pc = addr;
                }
            }
        }
    }

    pub fn run_ops(&mut self, ops: &[Op]) {
        while self.pc < ops.len() {
            let op = ops[self.pc].clone();
            self.run_op(op);
            self.increment_pc();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{lexer::Lexer, op::Op};

    use super::Interpreter;

    fn test_ops(ops: &[Op]) {
        Interpreter::interpret(ops)
    }

    #[test]
    fn can_run_loops() {
        let ops = Lexer::lex("+++[.-]".as_bytes());
        test_ops(&ops)
    }
}
