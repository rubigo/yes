# ![rubigo-coreutils](logo.png)
[![docs: published](https://img.shields.io/badge/docs-published-green.svg)](https://rubigo.github.io/yes/rubigo_yes) 
[![current tag](https://img.shields.io/github/tag/rubigo/yes.svg)](CHANGELOG.md) 
[![travis build status](https://travis-ci.org/rubigo/yes.svg?branch=master)](https://travis-ci.org/rubigo/yes)
[![codecov](https://codecov.io/gh/rubigo/yes/branch/master/graph/badge.svg)](https://codecov.io/gh/rubigo/yes)

This the `yes` utility, part of
[rubigo-coreutils](https://github.com/rubigo/coreutils), which endlessly repeats
its command line arguments. It needs at least rust `1.19.0` to work, since
the `eprintln!()` macro wasn't implemented before that.

## Features

It is both a library and a binary, so it can be embedded into other projects if
necessary. It should be compatible with both GNU `yes` and OpenBSD `yes`, except
that it has optional command-line flags `-h` for help and `-v` for version,
which those don't have. Thus, if you want it to repeat `-v`, instead of `yes -v`
you'd have to use `yes -- -v`. Additionally, it implements an optimisation from
GNU `yes`, by building a buffer (which is lmited by `BUFSIZE`) and sends that in 
one go, as opposed to sending the strings individually. 

## Dependencies

name | current version | description
---: | :-------------: | :----------
[clap](https://github.com/kbknapp/clap-rs) | [![Crates.io](https://img.shields.io/crates/v/clap.svg)](https://crates.io/crates/clap) | *Used to parse command-line arguments*

## Todo

- [X] make it work
- [X] make it pretty
- [ ] document everything
- [X] add continuous integration (appveyor, travis, etc.)
- [X] get test coverage over 80% (version 0.1.1)
- [ ] test on linux/bsd/windows (version 0.1.2)
- [ ] implement security features (`pledge` and `seccomp`) (version 0.1.3)
