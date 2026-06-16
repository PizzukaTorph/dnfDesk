#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub summary: String,
}

impl Package {
    pub fn new(name: impl Into<String>, version: impl Into<String>, summary: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            summary: summary.into(),
        }
    }
}

pub fn mock_packages() -> Vec<Package> {
    vec![
        Package::new("firefox", "latest", "Mozilla Firefox web browser"),
        Package::new("git", "latest", "Distributed version control system"),
        Package::new("neovim", "latest", "Vim-fork focused on extensibility and usability"),
        Package::new("rust", "latest", "Rust compiler toolchain"),
        Package::new("cargo", "latest", "Rust package manager"),
        Package::new("gtk4-devel", "latest", "GTK4 development files"),
        Package::new("libadwaita-devel", "latest", "Development files for libadwaita"),
        Package::new("dnf5", "latest", "Next-generation DNF package manager"),
    ]
}
