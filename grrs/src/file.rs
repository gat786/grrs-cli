use std::{error::Error, path::PathBuf};

use anyhow::Context;

#[allow(dead_code)]
pub fn read_file_default(path: &PathBuf) -> Result<String, Box<dyn Error>> {
    // maybe try implementing using BufReader so that all the file
    // does not get loaded into memory at once
    let result = std::fs::read_to_string(path);

    let content = match result {
        Ok(content) => content,
        Err(error) => {
            return Err(error.into());
        }
    };

    return Ok(content);
}

#[allow(dead_code)]
pub fn read_file_expect(path: &PathBuf) -> Result<String, Box<dyn Error>> {
    // expect lets you handle results, which are enums, when you dont want to deal with
    // resultant enums because you consider the failure to be most likely not happening
    // you can just do expect and handle it concisely without taking much space,
    // it will still panic but with less effort as a programmer required from you
    let result = std::fs::read_to_string(path).expect("could not read file");
    return Ok(result);
}

#[allow(dead_code)]
pub fn read_file_context(path: &PathBuf) -> Result<String, Box<dyn Error>> {
    let result = std::fs::read_to_string(path)
        .with_context(|| format!("could not read file: `{:?}`", path))?;
    return Ok(result);
}

#[allow(dead_code)]
pub fn read_file_question(path: &PathBuf) -> Result<String, Box<dyn Error>> {
    // ? - can also be used to eliminate the error return type, when used
    // if an error is received `?` will automatically return the error value, eliminating
    // the need to handle it in an err block
    let result = std::fs::read_to_string(path)?;
    return Ok(result);
}
