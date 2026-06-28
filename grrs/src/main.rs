use clap::Parser;

#[derive(Parser)]
struct CLI {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let args = CLI::parse();

    // maybe try implementing using BufReader so that all the file
    // does not get loaded into memory at once
    // expect lets you handle results, which are enums, when you dont want to deal with
    // resultant enums because you consider the failure to be most likely not happening
    // you can just do expect and handle it concisely without taking much space,
    // it will still panic but with less effort as a programmer required from you
    let result = std::fs::read_to_string(&args.path); // .expect("could not read file");

    match result {
        Ok(content) => {
            for line in content.lines() {
                if line.contains(&args.pattern) {
                    println!("{}", line);
                }
            }
            println!("pattern: {:?}, path: {:?}", args.pattern, args.path);
        }
        Err(error) => {
            println!("Oh noes!: {}", error);
        }
    }
}
