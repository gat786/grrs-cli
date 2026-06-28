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
