use std::path::PathBuf;

use clap::Parser;
mod file;

#[derive(Parser)]
struct CLI {
    pattern: String,
    path: std::path::PathBuf,
}

#[allow(dead_code)]
fn solve_directly(path: &PathBuf, pattern: &String) {
    match file::read_file_default(path) {
        Ok(content) => {
            for line in content.lines() {
                if line.contains(pattern) {
                    println!("{}", line);
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e)
        }
    }
}

#[allow(dead_code)]
fn solve_expect(path: &PathBuf, pattern: &String) {
    let result = file::read_file_expect(path);

    match result {
        Ok(content) => {
            for line in content.lines() {
                if line.contains(pattern) {
                    println!("{}", line)
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}

#[allow(dead_code)]
fn solve_expect_context(path: &PathBuf, pattern: &String) {
    let result = file::read_file_context(path);

    match result {
        Ok(content) => {
            for line in content.lines() {
                if line.contains(pattern) {
                    println!("{}", line)
                }
            }
        }
        Err(e) => {
            panic!("{}", e)
        }
    }
}

#[allow(dead_code)]
fn solve_question(path: &PathBuf, pattern: &String) {
    let result = file::read_file_question(path);

    match result {
        Ok(content) => {
            for line in content.lines() {
                if line.contains(pattern) {
                    println!("{}", line)
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}

fn main() {
    let args = CLI::parse();
    solve_expect_context(&args.path, &args.pattern);
}
