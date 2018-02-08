use semver::Version;
use std::process::Command;

use git_tag::GitTag;

pub struct GitTagParser {
    pub repository_path: String
}

impl GitTagParser {
    // Returns a vector of "from", "to" tuples for each tag found in the repository. The "from" revision is the previous semver
    // tag, the "to" revision is the current semver tag.
    pub fn get_version_tag_pairs(&self) -> Vec<(String, String)> {
        let mut from_version = self.get_root_ancestor();

        let mut tags = self.semver_tags();
        tags.sort();

        let mut tag_pairs: Vec<(String, String)> = tags.into_iter().map(|tag| {
            let old_from_version = from_version.clone();
            let to_version = tag.tag;
            from_version = to_version.clone();

            (old_from_version, to_version)
        }).collect();

        // TODO: Add the pair from "last tag to HEAD" if they do not point to the same rev.
        tag_pairs.reverse();

        tag_pairs
    }

    fn semver_tags(&self) -> Vec<GitTag> {
        let tags = self.get_tags();
        tags
            .into_iter()
            .filter(|tag| Version::parse(tag.replace("v", "").as_str()).is_ok())
            .map(|tag| GitTag {
                tag: tag.clone(),
                version: Version::parse(tag.replace("v", "").as_str()).ok().unwrap()
            })
            .collect()
    }

    // A lot of parameters to this one. 'git tag -l' is much simpler, but the problem is that it produces a list of
    // tags that is sorted in the wrong order. We want them in the order that they exist in the repo.
    fn get_tags_args() -> Vec<String> {
        [
            "for-each-ref",
            "--format=%(refname)",
            "refs/tags/*"
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
              .replace("refs/tags/", "")
              .trim()
              .to_string()
        ).rev().collect()
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
