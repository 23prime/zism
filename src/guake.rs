use std::process::Command;

use anyhow::{Context, Result};

pub fn is_inside_guake() -> bool {
    std::env::var("GUAKE_TAB_UUID").is_ok()
}

pub fn rename_tab(name: &str) -> Result<()> {
    Command::new("guake")
        .arg(format!("--rename-current-tab={name}"))
        .status()
        .context("Failed to run guake --rename-current-tab")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use serial_test::serial;

    use super::*;

    #[test]
    #[serial]
    fn is_inside_guake_returns_true_when_env_set() {
        unsafe { std::env::set_var("GUAKE_TAB_UUID", "some-uuid") };
        assert!(is_inside_guake());
        unsafe { std::env::remove_var("GUAKE_TAB_UUID") };
    }

    #[test]
    #[serial]
    fn is_inside_guake_returns_false_when_env_unset() {
        unsafe { std::env::remove_var("GUAKE_TAB_UUID") };
        assert!(!is_inside_guake());
    }
}
