use anyhow::{Result, bail};
use inquire::ui::{Attributes, Color, RenderConfig, StyleSheet};
use inquire::validator::Validation;
use inquire::{Select, Text};

fn render_config() -> RenderConfig<'static> {
    RenderConfig {
        prompt: StyleSheet::new().with_attr(Attributes::BOLD),
        selected_option: Some(
            StyleSheet::new()
                .with_fg(Color::Black)
                .with_bg(Color::LightCyan),
        ),
        ..RenderConfig::default()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Create,
    Attach,
    Delete,
}

impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::Create => write!(f, "Create new session"),
            Action::Attach => write!(f, "Attach to session"),
            Action::Delete => write!(f, "Delete session"),
        }
    }
}

pub fn available_actions(has_sessions: bool) -> Vec<Action> {
    if has_sessions {
        vec![Action::Create, Action::Attach, Action::Delete]
    } else {
        vec![Action::Create]
    }
}

pub fn select_action(has_sessions: bool) -> Result<Action> {
    let options = available_actions(has_sessions);
    let action = Select::new("Select an action:", options)
        .with_render_config(render_config())
        .prompt()?;
    Ok(action)
}

pub fn select_session(sessions: &[String]) -> Result<String> {
    if sessions.is_empty() {
        bail!("No sessions available to select.");
    }
    let session = Select::new("Select a session:", sessions.to_vec())
        .with_render_config(render_config())
        .prompt()?;
    Ok(session)
}

pub fn select_session_optional(sessions: &[String]) -> Result<Option<String>> {
    if sessions.is_empty() {
        return Ok(None);
    }
    let answer = Select::new("Select a session:", sessions.to_vec())
        .with_render_config(render_config())
        .prompt_skippable()?;
    Ok(answer)
}

pub fn input_session_name() -> Result<String> {
    let name = Text::new("Enter new session name:")
        .with_render_config(render_config())
        .with_validator(|input: &str| Ok(validate_session_name(input)))
        .prompt()?;
    Ok(name)
}

fn validate_session_name(name: &str) -> Validation {
    if name.trim().is_empty() {
        Validation::Invalid("Session name cannot be empty.".into())
    } else {
        Validation::Valid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn action_display_create() {
        assert_eq!(Action::Create.to_string(), "Create new session");
    }

    #[test]
    fn action_display_attach() {
        assert_eq!(Action::Attach.to_string(), "Attach to session");
    }

    #[test]
    fn action_display_delete() {
        assert_eq!(Action::Delete.to_string(), "Delete session");
    }

    #[test]
    fn available_actions_with_sessions_returns_all() {
        let actions = available_actions(true);
        assert_eq!(
            actions,
            vec![Action::Create, Action::Attach, Action::Delete]
        );
    }

    #[test]
    fn available_actions_without_sessions_returns_create_only() {
        let actions = available_actions(false);
        assert_eq!(actions, vec![Action::Create]);
    }

    #[test]
    fn select_session_optional_returns_none_when_empty() {
        let result = select_session_optional(&[]).unwrap();
        assert_eq!(result, None);
    }
}
