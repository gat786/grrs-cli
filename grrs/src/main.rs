use clap::Parser;

#[derive(Parser)]
struct CLI {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let args = CLI::parse();

    println!("pattern: {:?}, path: {:?}", args.pattern, args.path)
}
