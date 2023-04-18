use std::env;
use std::path::Path;
use walkdir::WalkDir;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("enter filename");
        std::process::exit(1);
    }

    let filename = &args[1];

    let start_dir = Path::new("/"); 
    find_file(start_dir, filename);
}

fn find_file(start_dir: &Path, filename: &str) {
    for entry in WalkDir::new(start_dir) {
        let entry = match entry {
            Ok(entry) => entry,
            Err(err) => {
                eprintln!("error searching path  {}", err);
                continue;
            }
        };

        if entry.file_name().to_string_lossy() == filename {
            println!("found file at : {}", entry.path().display());
        }
    }
}
