use std::{
    fs::File,
    io::{Error, Read},
};

pub fn read_input(path: &str) -> Result<String, Error> {
    let mut file_handle = File::open(path)?;
    let mut content = String::new();
    file_handle.read_to_string(&mut content)?;
    Ok(content)
}
