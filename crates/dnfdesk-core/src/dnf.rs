use anyhow::{Context, Result};
use std::process::Command;

use crate::{Package, TransactionPreview};

pub struct DnfClient;

impl DnfClient {
    pub fn new() -> Self {
        Self
    }

    pub fn search_packages(&self, query: &str) -> Result<Vec<Package>> {
        let query = query.trim();

        if query.len() < 2 {
            return Ok(Vec::new());
        }

        let output = Command::new("dnf5")
            .args(["search", query])
            .output()
            .context("failed to execute dnf5 search")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            anyhow::bail!("dnf5 search failed: {}", stderr);
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(parse_search_output(&stdout))
    }

    pub fn package_info(&self, package_name: &str) -> Result<String> {
        let output = Command::new("dnf5")
            .args(["info", package_name])
            .output()
            .context("failed to execute dnf5 info")?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        if !output.status.success() {
            anyhow::bail!("dnf5 info failed: {}", stderr);
        }

        Ok(stdout.to_string())
    }

    pub fn preview_install(&self, package_name: &str) -> Result<TransactionPreview> {
        let command = format!("dnf5 install --assumeno {}", package_name);

        let output = Command::new("dnf5")
            .args(["install", "--assumeno", package_name])
            .output()
            .context("failed to execute dnf5 install preview")?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        let mut combined = String::new();

        if !stdout.trim().is_empty() {
            combined.push_str(stdout.trim());
        }

        if !stderr.trim().is_empty() {
            if !combined.is_empty() {
                combined.push_str("\n\n--- STDERR ---\n");
            }
            combined.push_str(stderr.trim());
        }

        if combined.trim().is_empty() {
            combined = "No output returned by dnf5.".to_string();
        }

        Ok(TransactionPreview::new(command, combined))
    }
}

impl Default for DnfClient {
    fn default() -> Self {
        Self::new()
    }
}

pub fn parse_search_output(output: &str) -> Vec<Package> {
    output
        .lines()
        .filter_map(|line| {
            let line = line.trim();

            if line.is_empty()
                || line.starts_with("Updating")
                || line.starts_with("Repositories")
                || line.starts_with("Matched fields:")
            {
                return None;
            }

            let mut parts = line.splitn(2, '\t');

            let raw_name = parts.next()?.trim();
            let summary = parts.next().unwrap_or("").trim();

            let name = raw_name
                .split('.')
                .next()
                .unwrap_or(raw_name)
                .trim();

            if name.is_empty() {
                return None;
            }

            Some(Package::new(name, "available", summary))
        })
        .collect()
}
