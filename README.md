# DnfDesk

A GTK4/libadwaita graphical frontend for DNF5, written in Rust.

## Goals

- Fast package search
- Transparent DNF transactions
- Install/remove RPM packages
- Transaction previews
- Native Fedora experience

## Stack

- Rust
- GTK4
- libadwaita
- DNF5

## Run

Install dependencies:

```bash
sudo dnf install gtk4-devel libadwaita-devel
```

Then:

```bash
cargo run
```