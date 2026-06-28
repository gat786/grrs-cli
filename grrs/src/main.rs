use clap::Parser;

#[derive(Parser)]
struct CLI {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = CLI::parse();

    // maybe try implementing using BufReader so that all the file
    // does not get loaded into memory at once
    // expect lets you handle results, which are enums, when you dont want to deal with
    // resultant enums because you consider the failure to be most likely not happening
    // you can just do expect and handle it concisely without taking much space,
    // it will still panic but with less effort as a programmer required from you
    let result = std::fs::read_to_string(&args.path); // .expect("could not read file");
    // ? - can also be used to eliminate the error return type, when used
    // if an error is received `?` will automatically return the error value, eliminating
    // the need to handle it in an err block

    let content = match result {
        Ok(content) => content,
        Err(error) => {
            return Err(error.into());
        }
    };

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
    println!("pattern: {:?}, path: {:?}", args.pattern, args.path);
    return Ok(());
}
