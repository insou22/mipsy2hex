use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <file>", args[0]);
        std::process::exit(1);
    }

    let file_name = &args[1];

    let file_contents = match std::fs::read_to_string(file_name) {
        Ok(contents) => contents,
        Err(err) => {
            println!("Error: {}", err);
            std::process::exit(1);
        }
    };

    let iset = mipsy_lib::inst_set().expect("Failed to load instruction set");

    let binary = mipsy_lib::compile(&iset, &file_contents).expect("Failed to compile!");

    for opcode in binary.text {
        println!("{:08x}", opcode);
    }
}
