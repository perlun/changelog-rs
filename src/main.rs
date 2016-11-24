use std::env;
use std::process::Command;
use std::process::exit;

struct Settings<'a> {
    repository_path: &'a str,
    from_revision: &'a str,
    to_revision: &'a str
}

impl<'a> Settings<'a> {
    pub fn get_commits(self) {
        let range = format!("{}..{}", self.from_revision, self.to_revision);

        let output = Command::new("git")
                .arg("log")
                .arg("--oneline")
                .arg(&range)
                .current_dir(self.repository_path)
                .output().unwrap_or_else(|e| panic!("Failed to run 'git log' with error: {}", e));

        let stdout_output = String::from_utf8_lossy(&output.stdout);
        let lines = stdout_output
                .split('\n')
                .collect::<Vec<_>>();
        let mut lines_iterator = lines.iter();

        print!("## {}\n\n", self.to_revision);

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
}

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() == 4 {
        let settings = Settings {
            repository_path: &args[1],
            from_revision: &args[2],
            to_revision: &args[3]
        };
        settings.get_commits();
    }
    else {
        println!("Usage: {} <path> <from_revision> <to_revision>\n", args[0]);
        println!("The path must be a clone of valid git repository.");
        exit(1);
    }
}
