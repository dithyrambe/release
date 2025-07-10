use std::process::Command;

use anyhow::{Context, Result};

pub fn git_tag(tag: &str) -> Result<()> {
    let status = Command::new("git")
        .arg("tag")
        .arg(tag)
        .status()
        .context("Failed to execute git tag command")?;

    if !status.success() {
        return Err(anyhow::anyhow!(
            "Git tag command failed with exit code: {}",
            status.code().unwrap_or(-1)
        ));
    }

    Ok(())
}

pub fn git_pull() -> Result<()> {
    let status = Command::new("git")
        .arg("pull")
        .arg("--rebase")
        .status()
        .context("Failed to execute git pull command")?;

    if !status.success() {
        return Err(anyhow::anyhow!(
            "Git pull command failed with exit code: {}",
            status.code().unwrap_or(-1)
        ));
    }

    Ok(())
}

pub fn git_push() -> Result<()> {
    let status = Command::new("git")
        .arg("push")
        .arg("--tags")
        .status()
        .context("Failed to execute git push command")?;

    if !status.success() {
        return Err(anyhow::anyhow!(
            "Git push command failed with exit code: {}",
            status.code().unwrap_or(-1)
        ));
    }

    Ok(())
}

pub fn get_current_branch() -> Result<String> {
    let output = Command::new("git")
        .arg("branch")
        .arg("--show-current")
        .output()
        .context("Failed to get current branch")?;

    let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
    Ok(branch)
}
