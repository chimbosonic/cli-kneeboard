[![Crate][crate_img]][crate]
[![License][license_img]][license_file]
[![Crate Downloads][downloads_img]][crate]
![Build][actions]

# cli-kneeboard

`cli-kneeboard` is a tool that reads Markdown checklists and lets you programmatically run through them.

It is capable of saving the progress of a checklist as TOML allowing you to have reusable to-do lists.

When the app exits it will close with an ExitCode equal to the number of unresolved checklist items. This means you can use it as a git hook [example here](./hooks/pre-commit).

[![asciicast](https://asciinema.org/a/8tVyN78hH29YfFkIPF2YW91Dp.svg)](https://asciinema.org/a/8tVyN78hH29YfFkIPF2YW91Dp)

## Installation

### From binaries

- Go to [releases](https://github.com/chimbosonic/cli-kneeboard/releases) and download the latest binary for your architecture.
- Extract the file and copy the binary to your `PATH`.

### From source on crates.io

- Run `cargo install cli-kneeboard`

### From source on Github.com

- Clone or download the repo
- Use `cargo run -- [OPTIONS]` to run the app in debug mode
- Or compile it with `cargo build --release` and get the binary from `./target/release/kneeboard`

## Usage

```bash
$ kneeboard --help
cli-kneeboard 0.1.0
Alexis Lowe <agl314@chimbosonic.com>
Checklist Tool

USAGE:
    kneeboard [OPTIONS] --checklist-path <CHECKLIST_PATH>

OPTIONS:
    -c, --checklist-path <CHECKLIST_PATH>    Path to the checklist
    -h, --help                               Print help information
    -s, --save                               Save progress of the checklist
    -v, --verbose                            Turn debugging information on
    -V, --version                            Print version information
```

- `--checklist-path` || `-c` is required and is the path to the Markdown checklist you want to load
- `--save` || `-s` tells `cli-kneeboard` to save the progress of the checklist in the same folder as the checklist named `.<checklist-name>.kb.toml`
- `--verbose` || `-v` lets you set verbosity the more `v`'s provided the higher the verbosity

`cli-kneeboard` will read in a checklist following the specification described in [SPEC.md](./SPEC.md)

<!-- Badges -->
[crate]: https://crates.io/crates/cli-kneeboard "Crate Link"
[crate_img]: https://img.shields.io/crates/v/cli-kneeboard.svg?logo=rust "Crate Page"
[downloads_img]: https://img.shields.io/crates/dv/cli-kneeboard.svg?logo=rust "Crate Downloads"
[license_file]: https://github.com/chimbosonic/cli-kneeboard/blob/master/LICENSE "License File"
[license_img]: https://img.shields.io/crates/l/cli-kneeboard.svg "License Display"
[actions]: https://github.com/chimbosonic/cli-kneeboard/actions/workflows/build.yml/badge.svg
