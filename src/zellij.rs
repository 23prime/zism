use std::os::unix::process::CommandExt;
use std::process::Command;

use anyhow::{Context, Result, bail};

pub fn parse_sessions(output: &str) -> Vec<String> {
    output
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect()
}

pub fn build_create_args(name: &str) -> Vec<String> {
    vec!["--session".to_string(), name.to_string()]
}

pub fn build_attach_args(name: &str) -> Vec<String> {
    vec!["attach".to_string(), name.to_string()]
}

pub fn build_delete_args(name: &str) -> Vec<String> {
    vec![
        "delete-session".to_string(),
        "-f".to_string(),
        name.to_string(),
    ]
}

pub fn list_sessions() -> Result<Vec<String>> {
    let output = Command::new("zellij")
        .args(["list-sessions", "--short", "--no-formatting"])
        .output()
        .context("Failed to run zellij. Is it installed?")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        if stderr.contains("No active zellij sessions found") {
            return Ok(Vec::new());
        }
        bail!("zellij list-sessions failed: {stderr}");
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(parse_sessions(&stdout))
}

/// Creates a new Zellij session via `Command::exec()`, which replaces the
/// current process with Zellij on success. This means the function never
/// returns `Ok(())` — it only returns `Err` if the exec fails. Callers
/// should treat the `Result<()>` as representing only the error path.
pub fn create_session(name: &str) -> Result<()> {
    let err = Command::new("zellij").args(build_create_args(name)).exec();
    bail!("Failed to exec zellij: {err}");
}

/// Attaches to an existing Zellij session via `Command::exec()`, which
/// replaces the current process with Zellij on success. This means the
/// function never returns `Ok(())` — it only returns `Err` if the exec
/// fails. Callers should treat the `Result<()>` as representing only the
/// error path.
pub fn attach_session(name: &str) -> Result<()> {
    let err = Command::new("zellij").args(build_attach_args(name)).exec();
    bail!("Failed to exec zellij: {err}");
}

pub fn delete_session(name: &str) -> Result<()> {
    let status = Command::new("zellij")
        .args(build_delete_args(name))
        .status()
        .context("Failed to run zellij delete-session")?;

    if !status.success() {
        bail!("Failed to delete session '{name}'");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_sessions_returns_session_names() {
        let output = "my-project\ndev-server\ndotfiles\n";
        let sessions = parse_sessions(output);
        assert_eq!(sessions, vec!["my-project", "dev-server", "dotfiles"]);
    }

    #[test]
    fn parse_sessions_trims_whitespace() {
        let output = "  my-project  \n  dev-server  \n";
        let sessions = parse_sessions(output);
        assert_eq!(sessions, vec!["my-project", "dev-server"]);
    }

    #[test]
    fn parse_sessions_skips_empty_lines() {
        let output = "my-project\n\n\ndev-server\n";
        let sessions = parse_sessions(output);
        assert_eq!(sessions, vec!["my-project", "dev-server"]);
    }

    #[test]
    fn parse_sessions_returns_empty_for_empty_input() {
        let sessions = parse_sessions("");
        assert!(sessions.is_empty());
    }

    #[test]
    fn build_create_args_returns_correct_args() {
        let args = build_create_args("my-session");
        assert_eq!(args, vec!["--session", "my-session"]);
    }

    #[test]
    fn build_attach_args_returns_correct_args() {
        let args = build_attach_args("my-session");
        assert_eq!(args, vec!["attach", "my-session"]);
    }

    #[test]
    fn build_delete_args_returns_correct_args() {
        let args = build_delete_args("my-session");
        assert_eq!(args, vec!["delete-session", "-f", "my-session"]);
    }
}
