use semver::Version;
use std::process::Command;

pub struct GitTagParser {
    pub repository_path: String
}

impl GitTagParser {
    // Returns a vector of "from", "to" tuples for each tag found in the repository. The "from" revision is the previous semver
    // tag, the "to" revision is the current semver tag.
    pub fn get_version_tag_pairs(&self) -> Vec<(String, String)> {
        let mut from_version = self.get_root_ancestor();
        let mut tag_pairs: Vec<(String, String)> = self.semver_tags().into_iter().rev().map(|tag| {
            let old_from_version = from_version.clone();
            let to_version = tag;
            from_version = to_version.clone();

            (old_from_version, to_version)
        }).collect();

        // TODO: Add the pair from "last tag to HEAD" if they do not point to the same rev.
        tag_pairs.reverse();

        tag_pairs
    }

    fn semver_tags(&self) -> Vec<String> {
        let tags = self.get_tags();
        tags.into_iter().filter(|e| match Version::parse(e) {
            Ok(_) => true,
            Err(_) => false
        }).collect()
    }

    // A lot of parameters to this one. 'git tag -l' is much simpler, but the problem is that it produces a list of
    // tags that is sorted in the wrong order. We want them in the order that they exist in the repo.
    fn get_tags_args() -> Vec<String> {
        [
            "log",
            "--oneline",
            "--decorate",
            "--no-walk",
            "--tags",
            "--pretty=%D"
        ].iter().map(|e| e.to_string()).collect()
    }

    fn get_tags(&self) -> Vec<String> {
        let output = Command::new("git")
            .args(&GitTagParser::get_tags_args())
            .current_dir(&self.repository_path)
            .output()
            .unwrap_or_else(|e| panic!("Failed to run 'git tag' with error: {}", e));
        let output_stdout = String::from_utf8_lossy(&output.stdout);
        let output_lines = output_stdout.split('\n');

        output_lines.map(|e|
            e.to_string()
              .split(':')
              .last()
              .unwrap()
              .trim()
              .to_string()
        ).collect()
    }

    fn get_root_ancestor(&self) -> String {
        let output = Command::new("git")
            .args(&[
                "rev-list",
                "--max-parents=0",
                "HEAD"
            ])
            .current_dir(&self.repository_path)
            .output()
            .unwrap_or_else(|e| panic!("Failed to run 'git tag' with error: {}", e));
        let output_stdout = String::from_utf8_lossy(&output.stdout);
        output_stdout.trim().to_string()
    }
}
