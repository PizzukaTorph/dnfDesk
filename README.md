# DnfDesk

DnfDesk is a modern GTK4/libadwaita frontend for DNF5, written in Rust.

## Current milestone

This version includes:

- Rust workspace
- GTK4/libadwaita frontend
- real package search through `dnf5 repoquery`
- package selection
- install preview through `dnf5 install --assumeno`
- textual preview/log panel

## Fedora setup

```bash
sudo dnf install rust cargo gtk4-devel libadwaita-devel gcc pkgconf-pkg-config dnf5
```

## Run

```bash
cargo run -p dnfdesk-gtk
```

## Notes

The DNF calls are currently synchronous. This is acceptable for the prototype, but the next step should move DNF operations to background tasks so the UI never blocks.
