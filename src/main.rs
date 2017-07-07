#![deny(unused_imports)]
#![deny(unused_variables)]

extern crate semver;
extern crate clap;

mod changelog_generator;
mod git_tag_parser;

use clap::{App, Arg};
use changelog_generator::ChangelogGenerator;
use git_tag_parser::GitTagParser;

fn main() {
    let matches = App::new("changelog-rs")
                          .version("0.1.0")
                          .author("Per Lundberg <pelrun@gmail.com>")
                          .about("Trivial Rust-based CHANGELOG.md generation tool")
                          .arg(Arg::with_name("REPOSITORY_PATH")
                               .help("Sets the path of the repository to generate the changelog for. Defaults to the current directory.")
                               .index(1))
                          .arg(Arg::with_name("FROM_REVISION")
                              .help("The starting revision (usually the previous SemVer version).")
                              .index(2))
                          .arg(Arg::with_name("TO_REVISION")
                              .help("The ending revision (the current/latter SemVer version).")
                              .index(3))
                          .get_matches();

    let repository_path = String::from(matches.value_of("REPOSITORY_PATH").unwrap_or("."));

    if matches.is_present("FROM_REVISION") && matches.is_present("TO_REVISION") {
        let from_revision = String::from(matches.value_of("FROM_REVISION").unwrap());
        let to_revision = String::from(matches.value_of("TO_REVISION").unwrap());

        let generator = ChangelogGenerator {
            repository_path: repository_path,
            from_revision: from_revision,
            to_revision: to_revision
        };
        generator.generate_changelog();
    }
    else {
        generate_changelog_for_folder(repository_path);
    }
}

fn generate_changelog_for_folder(repository_path: String) {
    let git_tag_parser = GitTagParser {
        repository_path: repository_path.clone()
    };

    let version_tag_pairs = git_tag_parser.get_version_tag_pairs();

    for (from_tag, to_tag) in version_tag_pairs.into_iter() {
        let generator = ChangelogGenerator {
            repository_path: repository_path.clone(),
            from_revision: from_tag,
            to_revision: to_tag
        };
        generator.generate_changelog();
    }
}
