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
    let matches =
        App::new("changelog-rs")
            .version("0.3.0")
            .author("Per Lundberg <perlun@gmail.com> and others")
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
            .arg(Arg::with_name("latest")
                .long("latest")
                .help("Generate the changelog for the latest version only"))
            .arg(Arg::with_name("to-alias")
                .long("to-alias")
                .takes_value(true)
                .help("Revision to show in place of TO_REVISION (e.g. a tag about to be created)"))
            .get_matches();

    let repository_path = String::from(matches.value_of("REPOSITORY_PATH").unwrap_or("."));

    if matches.is_present("latest") {
        generate_latest_version_changelog_for_folder(repository_path);
    }
    else if matches.is_present("FROM_REVISION") && matches.is_present("TO_REVISION") {
        let from_revision = String::from(matches.value_of("FROM_REVISION").unwrap());
        let to_revision = String::from(matches.value_of("TO_REVISION").unwrap());
        let to_alias = matches.value_of("to-alias").map_or(to_revision.clone(), String::from);

        let generator = ChangelogGenerator {
            repository_path: repository_path,
            from_revision: from_revision,
            to_revision: to_revision,
            to_alias: to_alias
        };
        generator.generate_changelog();
    }
    else {
        generate_full_changelog_for_folder(repository_path);
    }
}

fn generate_full_changelog_for_folder(repository_path: String) {
    let git_tag_parser = GitTagParser {
        repository_path: repository_path.clone()
    };

    let version_tag_pairs = git_tag_parser.get_version_tag_pairs();

    for (from_tag, to_tag) in version_tag_pairs.into_iter() {
        let generator = ChangelogGenerator {
            repository_path: repository_path.clone(),
            from_revision: from_tag,
            to_revision: to_tag.clone(),
            to_alias: to_tag.clone()
        };
        generator.generate_changelog();
    }
}

fn generate_latest_version_changelog_for_folder(repository_path: String) {
    let git_tag_parser = GitTagParser {
        repository_path: repository_path.clone()
    };

    // Not the most intelligent algorithm in the world (getting all the tag pairs and then throwing away all but the first), but
    // for reasonable number of tags this shouldn't be a big performance impact overall.
    let version_tag_pairs = git_tag_parser.get_version_tag_pairs();
    let latest_tag_pair = version_tag_pairs.first().unwrap();
    let &(ref from_tag, ref to_tag) = latest_tag_pair;

    let generator = ChangelogGenerator {
        repository_path: repository_path.clone(),
        from_revision: from_tag.clone(),
        to_revision: to_tag.clone(),
        to_alias: to_tag.clone()
    };
    generator.generate_changelog();
}
