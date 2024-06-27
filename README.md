# semver-extra

[![crates.io](https://img.shields.io/crates/v/semver-extra)](https://crates.io/crates/semver-extra)

Extras to help when working with versions, built on top of the [semver](https://crates.io/crates/semver) crate, complete with a CLI tool.

## CLI

This project also publishes a binary application for use on the command line.

### Installation

For now, crates.io is the only place this is being distributed.

```
cargo install semver-extra
```

### Usage

```
A Rust implementation of the https://semver.org/ specification

Usage: semver [VERSION] [COMMAND]

Commands:
  increment  [aliases: i]
  get        [aliases: g]
  help       Print this message or the help of the given subcommand(s)

Arguments:
  [VERSION]


Options:
  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

### Example

```
```

## Development

### Prerequisites

- [nix](https://nixos.org/download.html)
- [nix flakes](https://nixos.wiki/wiki/Flakes#Enable_flakes)

### How-to

Create the development shell environment. Necessary to run all other commands.

```shell
nix develop
```

Build with cargo.

```shell
just build
```

Check the code with cargo's built-in fast static analysis.

```shell
just check
```

Remove build files.

```shell
just clean
```

Format the code.

```shell
just format
```

Check the code with clippy for better static analysis.

```shell
just lint
```

Run the application.

```shell
just run
```

Run tests with cargo's built-in test runner.

```shell
just test
```

Watch for code changes and rebuild.

```shell
just watch
```

All `just` commands can accept additional command line arguments after a `--`.

For example: run the application with a flag to report the version.

```shell
just run -- --version
```

#### Tips and Recommendations

##### Open IDE from Development Shell

To get linking to rust binaries in your IDE, you should open the development shell from your terminal and then open your IDE
from that shell session. This will carry over the development shell's environment into your IDE.

For example if you work with VSCode.

```shell
cd path/to/this/project
nix develop
code .
```

By doing this, you can install the rust-analyzer VSCode extension and it will work properly since it will be able to point to
the correct rust binaries and libraries. You will also have access in VSCode to any packages installed by the nix flake.

## To Do

- [ ] Dockerfile
- [ ] Github Action
- [ ] Support `#![no-std]`
- [ ] Support prerelease version incrementing
- [ ] Support range operations
- [ ] Rip off other features of [node-semver](https://github.com/npm/node-semver) if we find they are actually valuable
