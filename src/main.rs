mod banner;
mod guake;
mod ui;
mod zellij;

use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
#[command(version)]
struct Args {
    /// Number of candidates to display at once
    #[arg(long, default_value_t = 24)]
    page_size: usize,

    /// Rename Guake tab to session name on create/attach
    #[arg(long)]
    guake: bool,

    /// Print banner and exit
    #[arg(long, conflicts_with = "no_banner")]
    banner: bool,

    /// Suppress banner display
    #[arg(long, conflicts_with = "banner")]
    no_banner: bool,
}

fn is_inside_zellij() -> bool {
    std::env::var("ZELLIJ_SESSION_NAME").is_ok()
}

fn run(args: &Args) -> Result<()> {
    if !args.no_banner {
        banner::print_banner();
    }

    if args.banner {
        return Ok(());
    }

    if is_inside_zellij() {
        eprintln!("Already inside a Zellij session. Please run zism from outside Zellij.");
        std::process::exit(1);
    }

    let sessions = zellij::list_sessions()?;
    let has_sessions = !sessions.is_empty();

    match ui::select_action(has_sessions)? {
        ui::Action::Create => {
            let name = ui::input_session_name()?;
            if args.guake && guake::is_inside_guake() {
                guake::rename_tab(&name)?;
            }
            zellij::create_session(&name)?;
        }
        ui::Action::CreateWithDir => {
            let cwd = ui::input_directory(args.page_size)?;
            let name = cwd
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default();
            if args.guake && guake::is_inside_guake() {
                guake::rename_tab(&name)?;
            }
            zellij::create_session_with_dir(&name, &cwd)?;
        }
        ui::Action::Attach => {
            let session = ui::select_session(&sessions)?;
            if args.guake && guake::is_inside_guake() {
                guake::rename_tab(&session)?;
            }
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
    let args = Args::parse();
    run(&args)
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
