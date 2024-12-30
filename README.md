# cgp-exploration

Exploration of [Context Generic Programming](https://contextgeneric.dev/blog/early-preview-announcement/) in Rust. 

Main branch contains the previous pure trait based implementation.

The CGP has been trimmed down and will gradually convert the integration implementation 
into context based programming.


## Build commands

Cargo build works as expected. However, Bazel is configured as primary build system for this project.
Because not everyone is familiar with Bazel, I made a makefile to simplify all build related tasks.

```text
    make build          Builds the code base incrementally (fast) for dev.
    make current        Builds the current target incrementally (fast) defined in current.txt.
    make doc            Builds documentation for the project.
    make format         Formats call code according to cargo fmt style.
    make lint           Lints and formats the code of the project.
    make fix            Fixes linting issues as reported by clippy.
    make test           Tests across all crates.
    make vendor         Vendors all Bazel managed Rust dependencies to folder thirdparty.
```

For more details on the project build configuration, please read the [BUILD.md file](BUILD.md).

## Licence
This project is licensed under the [MIT license](LICENSE).

## Author
* [Marvin Hansen](https://github.com/marvin-hansen)
* Contact: https://deepcausality.com/contact/ 
* Github GPG key ID: 369D5A0B210D39BC
