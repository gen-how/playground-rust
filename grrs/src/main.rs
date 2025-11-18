struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let mut args = std::env::args();
    // Skips the first argument which is the program name.
    let _ = args.next();
    let pattern = args.next().expect("error: No pattern given");
    let path = args.next().expect("error: No path given");

    let cli = Cli {
        pattern: pattern,
        path: std::path::PathBuf::from(path),
    };

    println!("pattern={:?}, path={:?}", cli.pattern, cli.path);
}
