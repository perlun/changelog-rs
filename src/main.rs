#![deny(unused_imports)]

use std::env;
use std::process::Command;
use std::process::exit;

struct Changelog {
    repository_path: String,
    from_revision: String,
    to_revision: String,
    foo: String
}

impl Changelog {
    pub fn generate_changelog(self) {
        let output = self.get_log_output();
        let lines = Changelog::get_lines_from(&output);
        let mut lines_iterator = lines.iter();

        println!("## {}", self.to_revision);
        print!("[Full Changelog](https://github.com/{}/compare/{}...{})\n\n",
               self.get_repo_slug(), self.from_revision, self.to_revision);

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
        String::from_utf8_lossy(&output.stdout).into_owned()
    }

    fn range(&self) -> String {
        format!("{}..{}", self.from_revision, self.to_revision)
    }

    fn get_lines_from(output: &str) -> Vec<&str> {
        output
                .split('\n')
                .collect()
    }

    fn get_repo_slug(&self) -> String {
        let output = Command::new("git")
                .arg("remote")
                .arg("get-url")
                .arg("origin")
                .current_dir(&self.repository_path)
                .output()
                .unwrap_or_else(|e| panic!("Failed to run 'git log' with error: {}", e));

        let url = String::from_utf8_lossy(&output.stdout).into_owned();

        // The command output contains a trailing newline that we want to get rid of.
        let trimmed_url = url.trim();

        self.get_repo_slug_from(trimmed_url)
    }

    fn get_repo_slug_from(&self, url: &str) -> String {
        // This very simplistic and stupid algorithm works for repos of these forms:
        // https://github.com/dekellum/fishwife.git
        // git@github.com:chaos4ever/chaos.git
        let mangled_url = url.replace(":", "/").replace(".git", "");
        let mut url_parts: Vec<_> = mangled_url.split('/').collect();
        let repo_name = url_parts.pop().unwrap();
        let org_name = url_parts.pop().unwrap();

        format!("{}/{}", org_name, repo_name)
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
