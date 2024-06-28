# semver-extra

[![github](https://img.shields.io/badge/github-nicholaschiasson/semver-extra?logo=github)](https://github.com/nicholaschiasson/semver-extra)
[![crates.io](https://img.shields.io/crates/v/semver-extra?logo=rust)](https://crates.io/crates/semver-extra)
[![docs.rs](https://img.shields.io/docsrs/semver-extra?logo=docs.rs)](https://docs.rs/semver-extra)
[![build](https://github.com/nicholaschiasson/semver-extra/actions/workflows/build.yml/badge.svg)](https://github.com/nicholaschiasson/semver-extra/actions/workflows/build.yml)
[![license](https://img.shields.io/github/license/nicholaschiasson/semver-extra?logo=opensourceinitiative&logoColor=white)](https://github.com/nicholaschiasson/semver-extra?tab=MIT-1-ov-file#readme)

Helper functions for the [semver](https://crates.io/crates/semver) crate, complete with a CLI tool.

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
  increment  Increment a component of the version, resetting those of lower significance [aliases: i]
  get        Output a specific component of the version [aliases: g]
  help       Print this message or the help of the given subcommand(s)

Arguments:
  [VERSION]  The input semantic version. If omitted, input is taken from stdin

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Example

#### Attempt to parse input as a semantic version

```
semver hello.world
error: invalid value 'hello.world' for '[VERSION]': unexpected character 'h' while parsing major version number

For more information, try '--help'.
```

#### Bump minor version

```
semver 1.2.3 increment minor
1.3.0
```

#### Get prerelease component of version

```
semver 1.0.0-rc.12 get prerelease
rc.12
```

#### Pipe command output as semver input

```
echo 1.0.2 | semver i
1.0.3
```

## Docker

This project also publishes a docker image, exposing the CLI tool.

### Installation

You can pull the image from GitHub's container registry:

```
docker pull ghcr.io/nicholaschiasson/semver-extra:latest
```

Or for more convenience, you can reference the image in a docker compose file:

```yaml
---
services:
  semver:
    image: ghcr.io/nicholaschiasson/semver-extra:latest
```

For extra convenience, you can create an alias to the docker compose command:

```
echo 'alias semver="docker compose --file path/to/docker-compose.yml run --rm semver"' >> "${HOME}/.bashrc"
source "${HOME}"/.bashrc
```

After that, you should be able to simply run `semver` to invoke the container.

### Usage

The docker image entrypoint is the semver CLI binary itself, meaning the usage is the exact same as indicated above.

### Examples

#### Use docker image directly

```
docker run ghcr.io/nicholaschiasson/semver-extra 1.2.3 i major
2.0.0
```

#### Use docker compose service

```
docker compose run semver 12.34.56 i
12.34.57
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

- [x] Dockerfile
- [ ] Tests
- [ ] Github Action
- [ ] Support `#![no-std]`
- [ ] Support prerelease version incrementing
- [ ] Support range operations
- [ ] Rip off other features of [node-semver](https://github.com/npm/node-semver) if we find they are actually valuable
