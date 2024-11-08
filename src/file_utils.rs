use std::io::Read;
use std::fs::File;
use std::path::Path;

pub fn read_file_to_string(path: &Path) -> String {
    let mut file = File::open(path).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");
    contents
}
