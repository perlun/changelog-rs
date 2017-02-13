# changelog-rs

Trivial Rust-based CHANGELOG.md generation tool. The program uses git metadata (tags and commit messages) to generate the
changelog. Works especially well if you use the "squash on merge" or "rebase on merge" flow
[on GitHub](https://help.github.com/articles/about-merge-methods-on-github/). The former is for when you want to squash _all_
commits in a feature branch into a single commit on the base branch on merge, the latter is when you perhaps already have been
doing a manual rebase of your feature branch and want to retain these individual commits (because they perhaps relate to fixing two
different bugs, or whatever).

The tool do _not_ work well if you use the "traditional" merge mode, where all PR merges retain their individual commits _and_ adds
an extra "merge commit" that (to me) does not add any value at all, apart from cluttering the revision history. I strongly
discourage anyone from using this merge mode, it forces all other people working on the project to see a revision history with a
lot more details which are in the long run pretty useless. You care about _what_ the change was and _who_ did it. If you need more
details than that, the GitHub web interface gives you more details (like, the discussion that took place within the PR and the
history of how it looked like before merging, including the individual commits).

If you are stuck on "traditional" merge mode in your projects, you can still use the tool. It's just that its output will be less
readable because of all the individual commits that are retained + one extra merge commit for every single PR.

## Prerequisites

```shell
# This downloads the rust toolchain. You can omit this step if you already have a suitable Rust version installed.
curl https://sh.rustup.rs -sSf | sh
```

## How to install

```shell
$ cargo install changelog-rs
```

You will then have a `changelog-rs` in your path which you can run like this:

```shell
# If you want to generate a changelog entry for a particular version (from-revision and to-revision can be any git ref, i.e. a tag
# or a git commit SHA etc.
$ changelog-rs <repo> <from-revision> <to-revision>

# If you want to generate changelog entries for all versions that are tagged with SemVer compliant tags.
$ changelog-rs <repo>
```

## How to build and run

```shell
# This downloads the rust toolchain. You can omit this step if you already have a suitable Rust version installed.
curl https://sh.rustup.rs -sSf | sh

# Same parameter modes as above can be used.
cargo run <repo> <from-revision> <to-revision>
cargo run <repo>
```

"[SemVer](http://semver.org)-compliant tags" are expected to be on the form `0.1.0` or `v0.1.0` (the latter which is common in
certain communities.)
