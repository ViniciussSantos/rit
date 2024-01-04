fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let command = args.get(1).expect("No command provided");

    if command == "init" {
        let test = std::fs::metadata(".rit");
        if test.is_ok() {
            println!("Already a rit repository");
            return;
        }

        std::fs::create_dir(".rit").expect("Failed to create .rit directory");
        std::fs::create_dir(".rit/objects").expect("Failed to create .rit/objects directory");
        std::fs::create_dir(".rit/refs").expect("Failed to create .rit/refs directory");
    }
}
