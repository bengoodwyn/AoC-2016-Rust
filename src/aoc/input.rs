use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn read(filename: &str) -> String {
    let path = Path::new(&filename);
    let mut file = match File::open(path) {
        Err(why) => panic!("Failed to open {} because of {}", path.display(), why.description()),
        Ok(file) => file
    };
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Err(why) => panic!("Failed to read {} because of {}", path.display(), why.description()),
        Ok(_) => contents
    }
}