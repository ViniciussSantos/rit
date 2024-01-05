use std::fs::DirBuilder;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let command = args.get(1).expect("No command provided");

    if command == "init" {
        let test = std::fs::metadata(".rit");
        if test.is_ok() {
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
}
