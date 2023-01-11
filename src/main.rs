use pico_brainfuck::run;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        println!("Example Usage: pico_brainfuck <file-path>");
        return;
    }

    let src = std::fs::read(&args[1]).expect("Error reading file.");
    run(&src)
}
