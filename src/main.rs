use std::fs::DirBuilder;

mod blob;
mod file;

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
    let files_to_add = file::list_files(".");
    println!("Files to add: {:?}", files_to_add);

    for f in files_to_add {
        let content = file::read_file_content(f);
        let b: blob::Blob = blob::Blob::new(content);
        file::write_object_to_file(b);
    }
}
