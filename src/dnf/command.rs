use anyhow::Result;
use tokio::process::Command;

pub async fn run_dnf_command(args: &[&str]) -> Result<String> {
    let output = Command::new("dnf5")
        .args(args)
        .output()
        .await?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}