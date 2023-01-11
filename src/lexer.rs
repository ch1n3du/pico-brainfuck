use crate::op::Op;

pub struct Lexer<'a> {
    src: &'a [u8],
    current: usize,
    loops: Vec<usize>,
}

impl<'a> Lexer<'a> {
    pub fn lex(src: &[u8]) -> Vec<Op> {
        Lexer::new(src).lex_tokens()
    }

    fn new(src: &'a [u8]) -> Lexer<'a> {
        Lexer {
            src,
            current: 0,
            loops: Vec::new(),
        }
    }

    fn is_at_end(&self) -> bool {
        self.current > self.src.len() - 1
    }

    fn peek(&self) -> Option<u8> {
        if !self.is_at_end() {
            let chary = self.src[self.current];
            Some(chary)
        } else {
            None
        }
    }

    fn advance(&mut self) -> Option<u8> {
        if let Some(chary) = self.peek() {
            self.current += 1;
            Some(chary)
        } else {
            None
        }
    }

    fn matches(&mut self, expected: u8) -> bool {
        if let Some(c) = self.peek() {
            c == expected
        } else {
            false
        }
    }

    fn skip_whitespace(&mut self) {
        let valid_chars = b"[]+-<>,.";
        loop {
            if let Some(c) = self.peek() {
                if !valid_chars.contains(&c) {
                    self.advance();
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }

    fn lex_repeated(&mut self, expected: u8, tok: fn(usize) -> Op) -> Op {
        let mut count = 1;
        self.skip_whitespace();
        while self.matches(expected) {
            count += 1;
            self.advance();
            self.skip_whitespace();
        }
        tok(count)
    }

    fn lex_tokens(&mut self) -> Vec<Op> {
        let mut ops = Vec::new();

        while !self.is_at_end() {
            let chary = self.advance().unwrap();
            let op = match chary {
                b'>' => self.lex_repeated(b'>', Op::Right),
                b'<' => self.lex_repeated(b'<', Op::Left),
                b'+' => self.lex_repeated(b'+', Op::Plus),
                b'-' => self.lex_repeated(b'-', Op::Minus),
                b',' => Op::GetChar,
                b'.' => Op::PutChar,
                b'[' => {
                    // Push a dummy loop start
                    ops.push(Op::LoopStart(0));
                    let placeholder_index = ops.len() - 1;
                    // Add the index of the dummy to `self.loops` to be patched later.
                    self.loops.push(placeholder_index);
                    continue;
                }
                b']' => {
                    let start_index = self.loops.pop().expect("Loop end without start");
                    ops.push(Op::LoopEnd(start_index));

                    let end_index = ops.len() - 1;
                    ops[start_index] = Op::LoopStart(end_index);
                    continue;
                }
                _ => continue,
            };
            ops.push(op)
        }

        if !self.loops.is_empty() {
            panic!("Unclosed loop at {}", self.loops.pop().unwrap())
        }

        ops
    }
}

#[cfg(test)]
mod tests {
    use crate::op::Op;

    use super::Lexer;

    #[test]
    fn can_lex_everything() {
        let src = "[<>+-,.]";
        let ops = Lexer::lex(src.as_bytes());
        assert_eq!(ops.len(), 8)
    }

    #[test]
    fn can_lex_repitions() {
        let src = "[<<>+++++-----,.]";
        let ops = Lexer::lex(src.as_bytes());
        assert_eq!(
            ops,
            vec![
                Op::LoopStart(7),
                Op::Left(2),
                Op::Right(1),
                Op::Plus(5),
                Op::Minus(5),
                Op::GetChar,
                Op::PutChar,
                Op::LoopEnd(0)
            ]
        )
    }

    #[test]
    fn can_lex_with_spaces() {
        let src = "- - - - -";
        let ops = Lexer::lex(src.as_bytes());
        assert_eq!(ops[0], Op::Minus(5))
    }

    #[test]
    fn can_lex_nested_loops() {
        let src = "+++++[[-]]";
        let ops = Lexer::lex(src.as_bytes());
        assert_eq!(
            ops,
            vec![
                Op::Plus(5),
                Op::LoopStart(5),
                Op::LoopStart(4),
                Op::Minus(1),
                Op::LoopEnd(2),
                Op::LoopEnd(1)
            ]
        )
    }

    #[test]
    fn can_lex_hello_world() {
        let src = "++++++++[>++++]";
        let ops = Lexer::lex(src.as_bytes());
        assert_eq!(
            ops,
            vec![
                Op::Plus(8),
                Op::LoopStart(4),
                Op::Right(1),
                Op::Plus(4),
                Op::LoopEnd(1)
            ]
        )
    }
}
