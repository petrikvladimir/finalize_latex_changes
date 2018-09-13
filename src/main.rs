use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use std::io::prelude::*;

mod finalise_latex_changes;


/// Read file from the given path and return a String constructed from a file's content
fn read_file(path: &Path) -> String {
    let file = File::open(path).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    return contents;
}

fn main() {
    let file = Path::new("example/test_input.tex");
    let text = read_file(file);

    let out = finalise_latex_changes::filter_text(&text);

    println!("{}", out);
}
