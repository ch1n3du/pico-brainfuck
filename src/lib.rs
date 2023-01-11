use interpreter::Interpreter;
use lexer::Lexer;

mod interpreter;
mod lexer;
mod op;
// 30_000

pub fn run(src: &[u8]) {
    let ops = Lexer::lex(src);

    println!("{:?}", &ops[0..4]);
    Interpreter::interpret(&ops)
}
