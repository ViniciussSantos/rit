use std::fs;

use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::io::prelude::*;

use crate::blob::Blob;

pub fn get_gitignore_files() -> Option<Vec<String>> {
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

pub fn list_files(dir: &str) -> Vec<String> {
    let mut files = Vec::new();

    let mut gitignore_files = match get_gitignore_files() {
        Some(v) => v,
        None => vec![],
    };

    gitignore_files.append(&mut vec!["./.git".to_string(), "./.rit".to_string()]);

    for entry in std::fs::read_dir(dir).expect("Failed to read directory") {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();

        //TODO: make gitignore work with nested files and folders
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

pub fn read_file_content(file_path: String) -> String {
    let content: String = fs::read_to_string(file_path).unwrap().parse().unwrap();
    content
}

pub fn write_object_to_file(blob: Blob) {
    let path = ".rit/objects/".to_string() + &blob.oid[0..2] + "/" + &blob.oid[2..40];

    let content =
        "blob".to_string() + " " + &blob.data.as_bytes().len().to_string() + "\n" + &blob.data;

    fs::create_dir_all(".rit/objects/".to_string() + &blob.oid[0..2]).unwrap();

    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    let _ = encoder.write_all(content.as_bytes());

    fs::write(path, encoder.finish().unwrap()).unwrap();
}
