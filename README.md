[discord-shield]: https://img.shields.io/discord/1031520677787865108
[discord-invite]: https://discord.com/invite/xyaKBzarJs

[actions-shield]: https://github.com/bvanseg/ccake/actions/workflows/rust.yml/badge.svg
[actions-yml]: https://github.com/bvanseg/ccake/actions/workflows/rust.yml

[license]: https://img.shields.io/github/license/bvanseg/ccake
[code-size]: https://img.shields.io/github/languages/code-size/bvanseg/ccake

[rust-lang]: https://www.rust-lang.org/
[rust-shield]: https://img.shields.io/badge/Made%20with-Rust-1f425f.svg

# CCake

![GitHub][license]
[![rust-shield][]][rust-lang]
[![Rust][actions-shield]][actions-yml]
![GitHub code size in bytes][code-size]
[![discord-shield][]][discord-invite]

CCake is a command line tool written in Rust and designed to be a modern build tool + package manager for C/C++ projects.

## Goals

- To be easily understood by and accessible to beginner C/C++ developers.
- To familiarize beginner C/C++ developers with how C/C++'s build steps (preprocessing, compiling, linking, assembling, etc.) work.
- To keep CCake expressive in such a way that it feels familiar with other build and packaging tools such as `npm`, `yarn`, `gradle` and `cargo`.
- To make CCake cross-platform.
- To streamline the file structure for C/C++ projects into a more modern design pattern becoming increasingly and commonly used throughout the industry.
- To prioritize good project design patterns and habits over CCake's own backwards-compatibility (as the default).*
- To supplement modern compiler systems by making compiler input arguments more accessible to users (such as passing /src files into the compiler instead of the user manually doing so).
- To support backwards compatibility for compilers where necessary and possible.*

**Clarification: CCake as a tool does not seek to be backwards compatible in regards to running CCake's commands. However, maintaining backwards compatibility for compilers (such as GCC or Clang) is a goal that is in mind.*

## Non-Goals

- It is not a goal for CCake to support languages other than C/C++ at the time of writing.
- It is not a goal of CCake to implement custom scripting of any kind. While scripting is powerful, CCake should only ever be powerful enough to delegate such tasks to external software; i.e, if you want to tool your build environment around usage of python scripting, CCake should be able to run Python scripts in the pipeline through shell commands, but not much more than that.
- It is not a goal of the CCake project to preserve backwards compatibility (in regards to CCake) as the default behavior for CCake. In other tools such as with CMake, this method has proven over time to be detrimental to the longevity of CMake and the accessibility of newer developers, as CMake over time has improved and modernized to take better approaches to certain tasks, but has left the less optimal approaches as the defaults. Moreover, documentation online has grown to a point where older references now show sub-optimal approaches as opposed to modern, optimal approaches to project management. Therefore, it is an imperative of CCake to put modern designs at the forefront, and for backwards-compatibility to be supported through A) versioning and B) opt-in flags to CCake.

## Future Goals

These are far-future goals that are not goals at the moment, but are slated to become goals after CCake has satisified its original goals in a release environment.

- ~~An install system for compilers. Expected behavior would be to download compilers via `ccake install clang`, which would install the `clang` compiler to a `.ccake` folder on the user's system.~~ Implemented.
- An install system for third-party libraries in a manner similar to that of Java's or Node's library ecosystems. Expected behavior would be to download some library like GLFW using a command `ccake install glfw` without need to manually link/touch the library's downloaded files.
- Introducing a `[dependencies]` section within `ccake.toml` that would support a command such as `ccake install` to download all necessary library files, such that third parties do not have their files distributed through parties other than the maintainer/owner of said files.

## Build

- Install cargo for Rust.
- Clone the project and cd into the project directory.
- Run `cargo build` in the root directory of the project.

## Usage

- Use `cargo run -- <ccake args>` to test ccake. Example: `cargo run -- --version`

### Examples on Windows

- Run `cargo build` and then add the generated executable to your environment variables path. This will allow you to invoke the `ccake` command in the terminal.
- Run `ccake install mingw` to install a portable GCC compiler toolchain. This command will also set CCake's default C/C++ compiler paths to GCC/G++ respectively.
- `cd` into any of the projects found in the `examples` folder.
- Run `ccake build`. This should output an executable for the binary projects into an `out` folder.
- Run the executable.

### Examples on Linux

- Linux is not yet supported by CCake. PRs are welcome to add Linux support.
