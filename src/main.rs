use std::env;
use std::process::Command;
use std::process::exit;

pub fn get_commits(dir: &str, from_revision: &str, to_revision: &str) {
    let range = format!("{}..{}", from_revision, to_revision);

    let output = Command::new("git")
            .arg("log")
            .arg("--oneline")
            .arg(&range)
            .current_dir(dir)
            .output().unwrap_or_else(|e| panic!("Failed to run 'git log' with error: {}", e));

    let stdout_output = String::from_utf8_lossy(&output.stdout);
    let lines = stdout_output
            .split('\n')
            .collect::<Vec<_>>();
    let mut lines_iterator = lines.iter();

    print!("## {}\n\n", to_revision);

    loop {
        match lines_iterator.next() {
            Some(line) => {
                if line.is_empty() { return; }
                println!("* {}", line)
            },
            None => break
        }
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() == 4 {
        get_commits(&args[1], &args[2], &args[3]);
    }
    else {
        println!("Usage: {} <path> <from_revision> <to_revision>\n", args[0]);
        println!("The path must be a clone of valid git repository.");
        exit(1);
    }
}
