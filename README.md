# erd-biome-map-test
A test for laying out how biomes are created based on marjoris index (centrality to the suns), and minoris level (proximity to the void)

## Compiling

### Rust install
You will need the rust toolchain to build and run this program. Head to https://rust-lang.org/tools/install/ for information on how to install based on your platform.

### Nightly compiler
This project uses the nightly rust channel, for the latest shiny toys and performance improvements. If it isn't already, install the nightly toolchain:
```
$ rustup toolchain install nightly
```

### Bevy dependencies
Follow the instructions in the *Installing OS Dependencies* section of the bevy quick start guide to find out how to install the necessary OS-level dependencies, at https://bevy.org/learn/quick-start/getting-started/setup/

### Codegen component
Run the following command to install the nightly-only, faster codegen preview:

```
$ rustup component add rustc-codegen-cranelift-preview --toolchain nightly
```

### Linker
If you are compiling on Linux, make sure you have clang and mold installed. If it isn't, follow the right command below for your distro (if listed).

- Debian, Ubuntu or derivatives:
```
# apt install mold clang
```

- Fedora or derivatives:
```
# dnf install mold clang
```
- Arch Linux or derivatives:
```
# pacman -S mold clang
```

- Gentoo Linux or derivatives:
```
# emerge --ask sys-devel/mold llvm-core/clang
```

If your distro isn't listed, or doesn't ship either of these, you can install both of them from source, via their respective GitHub pages:

- mold:
https://github.com/rui314/mold.git

- clang:
https://github.com/llvm/llvm-project.git

NOTE: clang is installed as a dependency of the larger LLVM compiler infrastructure project

### Running the compiler

If you wish to try it out in release mode, simply run:

```
$ cargo run --release
```

If instead you want it in debug mode, omit the `--release` flag
```
$ cargo run
```

If you wish to compile and not run, substitute `run` for `build`

- release:
```
$ cargo build --release
```

- debug:
```
$ cargo build
```

The resulting binary and library files will be available in the auto-created `target/` folder in the project root, either in `target/release/`, or `target/debug/`, based on how you compiled it.