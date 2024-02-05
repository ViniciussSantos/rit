use core::str;
use std::fs::DirBuilder;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let command = args.get(1).expect("No command provided");

    match command.as_str() {
        "init" => init(),
        "commit" => commit(),
        _ => println!("Unknown command {}", command),
    }
}

fn init() {
    let rit_dir_metadata = std::fs::metadata(".rit");
    if rit_dir_metadata.is_ok() {
        println!("Already a rit repository");
        return;
    }

    for dir in [".rit/objects", ".rit/refs"] {
        DirBuilder::new()
            .recursive(true)
            .create(dir)
            .expect("Failed to create .rit directory");
    }
}

fn commit() {
    let files_to_add = list_files(".");
    println!("{:?}", files_to_add);
}

fn get_gitignore_files() -> Option<Vec<String>> {
    let gitignore_metadata = std::fs::metadata(".gitignore");
    if gitignore_metadata.is_err() {
        return None;
    }

    let buf = std::fs::read(".gitignore").unwrap();

    let mut filenames: Vec<String> = vec![];

    match std::str::from_utf8(&buf) {
        Ok(v) => {
            filenames.extend(
                v.split('\n')
                    .filter(|s| !s.is_empty() && !s.starts_with('#'))
                    .map(|s| s.to_string()),
            );
        }
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    Some(filenames)
}

fn list_files(dir: &str) -> Vec<String> {
    let mut files = Vec::new();

    let mut gitignore_files = match get_gitignore_files() {
        Some(v) => v,
        None => vec![],
    };

    gitignore_files.append(&mut vec!["./.git".to_string(), "./.rit".to_string()]);

    for entry in std::fs::read_dir(dir).expect("Failed to read directory") {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();

        let is_in_gitignore = gitignore_files.clone().into_iter().any(|gf| {
            path.starts_with(gf.clone()) || path.starts_with(["./".to_owned(), gf.clone()].join(""))
        });

        if path.is_file() && !is_in_gitignore {
            files.push(
                path.to_str()
                    .expect("Failed to convert path to string")
                    .to_string(),
            )
        } else if path.is_dir() {
            let mut subdir_files = list_files(path.to_str().unwrap());
            files.append(&mut subdir_files);
        }
    }
    files
}
