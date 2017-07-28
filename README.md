# ![rubigo-coreutils](img/logo.png)
[![docs: published](https://img.shields.io/badge/docs-published-green.svg)](https://rubigo.github.io/yes/rubigo_yes) 
[![current tag](https://img.shields.io/github/tag/rubigo/yes.svg)](CHANGELOG.md) 
[![travis build status](https://travis-ci.org/rubigo/yes.svg?branch=master)](https://travis-ci.org/rubigo/yes)



This the `yes` utility, which endlessly repeats its command line arguments. It
depends on at least rust 1.19.0 to work, since the `eprintln!()` macro wasn't
implemented before that.

## Todo

- [ ] add tests (version 0.1.1)
- [ ] test on linux/bsd/windows (version 0.1.2)
- [ ] add continuous integration (appveyor, travis, etc.)
    (version 0.1.3)
- [X] publish docs
- [ ] implement security features (`pledge` and `seccomp`)
