#![deny(unused_imports)]
#![deny(unused_variables)]

extern crate semver;

mod changelog_generator;
mod git_tag_parser;

use changelog_generator::ChangelogGenerator;
use git_tag_parser::GitTagParser;
use std::env;
use std::process::exit;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() == 2 {
        let ref repository_path = args[1];
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
            print!("\n");
        }
    } else if args.len() == 4 {
        let generator = ChangelogGenerator {
            repository_path: args[1].clone(),
            from_revision: args[2].clone(),
            to_revision: args[3].clone()
        };
        generator.generate_changelog();
    } else {
        println!("Usage: {} <path> [<from_revision> <to_revision>]\n",
                 args[0]);
        println!("The path must be a clone of valid git repository.");
        println!("If the 'from_revision' and 'to_revision' are not provided, a full log \
                  including all SemVer-tagged versions will be produced.");
        exit(1);
    }
}
