# DnfDesk

DnfDesk is a modern GTK/libadwaita graphical frontend for **DNF5**, written in **Rust**.

The goal is not to clone GNOME Software. DnfDesk aims to be a transparent, fast and technical package manager for Fedora users who want a native desktop UI without hiding what DNF is doing.

## Project vision

DnfDesk should feel like a modern Fedora-native equivalent of a technical package manager:

- search real RPM packages from enabled repositories
- inspect package metadata
- preview transactions before applying them
- install and remove packages through DNF5
- show the equivalent CLI command
- expose transaction output and logs clearly
- eventually manage updates, repositories and history

## Current status

Very early prototype.

The current codebase contains:

- a minimal GTK4/libadwaita application window
- a placeholder package search layout
- a first `dnf5` command wrapper
- separated modules for UI, DNF integration, package models and transactions

No real package operations are exposed from the GUI yet.

## MVP roadmap

### v0.1 - Package search

- main window
- search entry
- package results list
- call `dnf5 repoquery`
- display package name and summary

### v0.2 - Package details

- selected package details panel
- call `dnf5 info <package>`
- show version, repository, license, size and description

### v0.3 - Transaction preview

- simulate installation/removal
- use `dnf5 install --assumeno <package>`
- parse and display packages that would be installed, upgraded or removed

### v0.4 - Real transactions

- run privileged operations with `pkexec`
- install packages
- remove packages
- show command output and result state

### Future ideas

- update manager
- repository management
- transaction history
- rollback helpers where possible
- COPR discovery, with explicit warnings
- Fedora packaging

## Tech stack

- Rust
- GTK4
- libadwaita
- DNF5
- Tokio for process execution

## Fedora development setup

Install system dependencies:

```bash
sudo dnf install rust cargo gtk4-devel libadwaita-devel gcc pkgconf-pkg-config
```

Clone and run:

```bash
git clone https://github.com/PizzukaTorph/dnfDesk.git
cd dnfDesk
cargo run
```

## Design principles

### Be transparent

DnfDesk should show what it is going to do before doing it. Every transaction should expose the equivalent DNF command.

### Be native

The UI should follow GNOME/Fedora conventions using GTK4 and libadwaita.

### Be technical but safe

The application is intended for users who understand package management, but it should still prevent accidental destructive actions.

### Start simple

The first milestone intentionally avoids advanced abstractions. The priority is to get a working vertical slice before refining the architecture.

## Non-goals for now

- Flatpak management
- app store screenshots/reviews
- GNOME Software replacement
- distribution upgrades
- automatic COPR enablement
- advanced kernel management

## License

License not decided yet.
