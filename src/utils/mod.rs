use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn read_input(name: &str) -> String {
    let file_name = format!("inputs/{name}.txt");
    let path = Path::new(file_name.as_str());
    let display = path.display();
    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect(format!("Unable to read input string to file, path: {}", display).as_str());

    contents
}