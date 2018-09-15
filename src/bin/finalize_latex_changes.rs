#[macro_use]
extern crate clap;
extern crate finalize_latex_changes;


use std::path::Path;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = clap::App::from_yaml(yaml).get_matches();

    let input_filename = matches.value_of("INPUT").unwrap();
    let output_filename = matches.value_of("OUTPUT").unwrap_or(input_filename);

    println!("Going to process file {} and write result into {}", input_filename, output_filename);

    let input_path = Path::new(input_filename);
    let out_path = Path::new(output_filename);

    if input_path.is_dir() {
        panic!("Cannot process directory yet.");
//        todo add support for directory
    }

//    todo how to overwrite input file is output is not given?


    let mut f = finalize_latex_changes::Filter::new();

    if let Err(err) = f.process_file(input_path, out_path) {
        eprintln!("{}", err);
        std::process::exit(1);
    }

}
