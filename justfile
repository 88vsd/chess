alias cnp := create-new-project
alias inp := init-new-project
alias cnl := create-new-library
alias inl := init-new-library

alias b := build
alias r := run
alias bnc := bench
alias t := test
alias c := check
alias d := doc
alias cln := clean

alias cs := crate-search
alias i := install
alias li := list-installed

alias ur := update-rust
#alias ir := install-rust 

default:
    @just --list

#
# 🪄 STARTING A NEW PROJECT
#

# Creates a new Rust project with the given name.
create-new-project project_name:
    cargo new --bin {{project_name}}

# Creates a new Rust project in your current directory.
init-new-project:
    cargo init --bin

# Creates a new Rust library with the given name.
create-new-library library_name:
    cargo new --lib {{library_name}}

# Creates a new Rust library in your current directory.
init-new-library:
    cargo init --lib

#
# 🛠 GENERAL
#

# Builds your Rust project.
build:
    cargo build

# Executes your Rust project.
run:
    cargo run

# Executes the benchmarks of your Rust project.
bench:
    cargo bench

# Executes the tests of your Rust project.
test:
    cargo test

# Checks for errors in your Rust project.
check:
    cargo check

# Creates your Rust project documentation (use `cargo doc --open` to open it).
doc:
    cargo doc

# Removes the `target` directory.
clean:
    cargo clean

#
# 📦 CRATES
#

# Searches for crates on `crates.io`.
crate-search:
    cargo search

# Installs the specified crate.
install crate_name:
    cargo install {{crate_name}}

# Lists all the crates you installed.
list-installed:
    cargo install --list

#
# 🦀 INSTALLING/UPDATING RUST 
#

update-rust:
    rustup update

install-rust:
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh