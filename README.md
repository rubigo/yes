# ![rubigo-coreutils](img/logo.png)

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
