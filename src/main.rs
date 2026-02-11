mod ui;
mod zellij;

use anyhow::Result;

fn is_inside_zellij() -> bool {
    std::env::var("ZELLIJ_SESSION_NAME").is_ok()
}

fn run() -> Result<()> {
    if is_inside_zellij() {
        eprintln!("Already inside a Zellij session. Please run zism from outside Zellij.");
        std::process::exit(1);
    }

    let sessions = zellij::list_sessions()?;
    let has_sessions = !sessions.is_empty();

    match ui::select_action(has_sessions)? {
        ui::Action::Create => {
            let name = ui::input_session_name()?;
            zellij::create_session(&name)?;
        }
        ui::Action::Attach => {
            let session = ui::select_session(&sessions)?;
            zellij::attach_session(&session)?;
        }
        ui::Action::Delete => loop {
            let sessions = zellij::list_sessions()?;
            if sessions.is_empty() {
                break;
            }
            let Some(session) = ui::select_session_optional(&sessions)? else {
                break;
            };
            zellij::delete_session(&session)?;
            println!("Deleted session '{session}'");
        },
    }

    Ok(())
}

fn main() -> Result<()> {
    run()
}

#[cfg(test)]
mod tests {
    use serial_test::serial;

    use super::*;

    #[test]
    #[serial]
    fn is_inside_zellij_returns_true_when_env_set() {
        unsafe { std::env::set_var("ZELLIJ_SESSION_NAME", "test") };
        assert!(is_inside_zellij());
        unsafe { std::env::remove_var("ZELLIJ_SESSION_NAME") };
    }

    #[test]
    #[serial]
    fn is_inside_zellij_returns_false_when_env_unset() {
        unsafe { std::env::remove_var("ZELLIJ_SESSION_NAME") };
        assert!(!is_inside_zellij());
    }
}
