use std::env;
use std::process::Command;
use std::process::exit;
use std::process::Output;

struct Changelog {
    repository_path: String,
    from_revision: String,
    to_revision: String
}

impl Changelog {
    pub fn generate_changelog(self) {
        let output = self.get_log_output();
        let lines = Changelog::get_lines_from(&output);
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

    fn get_log_output(&self) -> String {
        let output = Command::new("git")
                .arg("log")
                .arg("--oneline")
                .arg(self.range())
                .current_dir(&self.repository_path)
                .output()
                .unwrap_or_else(|e| panic!("Failed to run 'git log' with error: {}", e));
        unsafe {
            String::from_utf8_unchecked(output.stdout)
        }
    }

    fn range(&self) -> String {
        format!("{}..{}", self.from_revision, self.to_revision)
    }

    fn get_lines_from(output: &str) -> Vec<&str> {
        output
                .split('\n')
                .collect()
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() == 4 {
        let changelog = Changelog {
            repository_path: args[1].clone(),
            from_revision: args[2].clone(),
            to_revision: args[3].clone()
        };
        changelog.generate_changelog();
    }
    else {
        println!("Usage: {} <path> <from_revision> <to_revision>\n", args[0]);
        println!("The path must be a clone of valid git repository.");
        exit(1);
    }
}
