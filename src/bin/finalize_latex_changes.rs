#[macro_use]
extern crate clap;
extern crate colored;
extern crate walkdir;
extern crate finalize_latex_changes;

use colored::*;
use std::path::PathBuf;
use walkdir::{WalkDir, DirEntry};

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name().to_str().map(|s| s.starts_with(".")).unwrap_or(false)
}

fn is_tex(entry: &DirEntry) -> bool {
    entry.file_name().to_str().map(|s| s.ends_with(".tex")).unwrap_or(false)
}

fn backup_extension() -> String {
    let start = std::time::SystemTime::now();
    let duration = start.duration_since(std::time::UNIX_EPOCH).expect("Time went backwards");
    let t = duration.as_secs() as f64 + duration.subsec_nanos() as f64 * 1e-9;
    String::from("tex_") + &t.to_string()
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = clap::App::from_yaml(yaml).get_matches();
    let input_filename = matches.value_of("INPUT").unwrap();
    let backup_dir = matches.value_of("backup_dir").unwrap_or(".backup_changes");

    println!("Creating a backup directory: {}", backup_dir.bold());
    if let Err(err) = std::fs::create_dir_all(backup_dir) {
        eprintln!("Cannot create a backup directory {}", err);
        std::process::exit(-1);
    }

    let walker = WalkDir::new(input_filename);
    walker.into_iter()
        .filter_entry(|e| !is_hidden(e))
        .filter_map(|e| e.ok())
        .filter(|e| is_tex(&e))
        .for_each(|e| {
            let in_path = e.into_path();
            let out_path = in_path.clone().with_extension("tex_changes");
            let backup_path = PathBuf::from(backup_dir)
                .join(in_path.file_name().unwrap_or(std::ffi::OsStr::new("unknown_name")))
                .with_extension(backup_extension());

            println!("Processing {}\n\t backup: {} \n\t temporary: {}",
                     in_path.display().to_string().bold(),
                     backup_path.display().to_string(),
                     out_path.display()
            );

            if let Err(err) = std::fs::copy(&in_path, backup_path) {
                eprintln!("Cannot create backup of the file. {}", err);
                std::process::exit(1);
            }

            let mut f = finalize_latex_changes::Filter::new();
            if let Err(err) = f.process_file(in_path.as_path(), out_path.as_path()) {
                eprintln!("{}", err);
                std::process::exit(1);
            }

            if let Err(err) = std::fs::rename(out_path, in_path) {
                eprintln!("Cannot replace original file with our temporary. {}", err);
                std::process::exit(1);
            }

            println!("\t {}{}", "added artifacts   : ".green(), f.num_added());
            println!("\t {}{}", "deleted artifacts : ".red(), f.num_deleted());
            println!("\t {}{}", "replaced artifacts: ".blue(), f.num_replaced());
        });
}
