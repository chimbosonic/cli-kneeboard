# cli-kneeboard

## Usage

- Clone the repo
- use `cargo run` to run the app

```bash
$ cargo run -- -h
   Compiling cli-kneeboard v0.1.0 (/home/chimbosonic/repos/github.com/cli-kneeboard)
    Finished dev [unoptimized + debuginfo] target(s) in 3.28s
     Running `target/debug/kneeboard -h`
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

----
#### N.B: Compile binaries and build pipeline T.B.C.

For now if you want a compile binary you can run:
```bash
cargo build --release
```
And get the binary from `./target/release/kneeboard`