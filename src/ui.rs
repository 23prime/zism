use std::path::{Path, PathBuf};

use anyhow::{Result, bail};
use inquire::ui::{Attributes, Color, RenderConfig, StyleSheet};
use inquire::validator::Validation;
use inquire::{Autocomplete, CustomUserError, Select, Text};

use crate::action::Action;

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

pub fn available_actions(has_sessions: bool) -> Vec<Action> {
    if has_sessions {
        vec![
            Action::CreateWithDir,
            Action::Create,
            Action::Attach,
            Action::Delete,
        ]
    } else {
        vec![Action::CreateWithDir, Action::Create]
    }
}

pub fn select_action(has_sessions: bool) -> Result<Action> {
    let options = available_actions(has_sessions);
    let action = Select::new("Select an action:", options)
        .with_render_config(render_config())
        .with_vim_mode(true)
        .prompt()?;
    Ok(action)
}

pub fn select_session(sessions: &[String], action: Action) -> Result<String> {
    if sessions.is_empty() {
        bail!("No sessions available to select.");
    }
    let session = Select::new("Select a session:", sessions.to_vec())
        .with_render_config(action.render_config())
        .with_vim_mode(true)
        .prompt()?;
    Ok(session)
}

pub fn select_session_optional(sessions: &[String], action: Action) -> Result<Option<String>> {
    if sessions.is_empty() {
        return Ok(None);
    }
    let answer = Select::new("Select a session:", sessions.to_vec())
        .with_render_config(action.render_config())
        .with_vim_mode(true)
        .prompt_skippable()?;
    Ok(answer)
}

pub fn input_session_name(action: Action) -> Result<String> {
    let name = Text::new("Enter new session name:")
        .with_render_config(action.render_config())
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

pub fn longest_common_prefix(items: &[String]) -> String {
    let Some(first) = items.first() else {
        return String::new();
    };
    let mut prefix_len = first.len();
    for item in &items[1..] {
        prefix_len = first
            .chars()
            .zip(item.chars())
            .take(prefix_len)
            .take_while(|(a, b)| a == b)
            .count();
    }
    first[..first
        .char_indices()
        .nth(prefix_len)
        .map_or(first.len(), |(i, _)| i)]
        .to_string()
}

#[derive(Clone)]
struct DirCompleter {
    home: PathBuf,
}

impl DirCompleter {
    fn new(home: PathBuf) -> Self {
        Self { home }
    }

    fn list_dirs(&self, dir: &Path) -> Vec<String> {
        let Ok(entries) = std::fs::read_dir(dir) else {
            return Vec::new();
        };
        let mut dirs: Vec<String> = entries
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().map(|ft| ft.is_dir()).unwrap_or(false))
            .filter_map(|e| {
                let name = e.file_name().to_string_lossy().to_string();
                if name.starts_with('.') {
                    None
                } else {
                    Some(name)
                }
            })
            .collect();
        dirs.sort();
        dirs
    }
}

impl Autocomplete for DirCompleter {
    fn get_suggestions(&mut self, input: &str) -> Result<Vec<String>, CustomUserError> {
        let (dir, prefix) = if input.is_empty() || input.ends_with('/') {
            (self.home.join(input), String::new())
        } else {
            let path = Path::new(input);
            let parent = path
                .parent()
                .map(|p| self.home.join(p))
                .unwrap_or_else(|| self.home.clone());
            let file = path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            (parent, file)
        };

        let base = if input.is_empty() || !input.contains('/') {
            String::new()
        } else if input.ends_with('/') {
            input.to_string()
        } else {
            let parent = Path::new(input).parent().unwrap_or(Path::new(""));
            if parent.as_os_str().is_empty() {
                String::new()
            } else {
                format!("{}/", parent.display())
            }
        };

        let suggestions = self
            .list_dirs(&dir)
            .into_iter()
            .filter(|name| prefix.is_empty() || name.starts_with(&prefix))
            .map(|name| format!("{base}{name}"))
            .collect();

        Ok(suggestions)
    }

    fn get_completion(
        &mut self,
        input: &str,
        highlighted_suggestion: Option<String>,
    ) -> Result<Option<String>, CustomUserError> {
        if let Some(suggestion) = highlighted_suggestion {
            return Ok(Some(format!("{suggestion}/")));
        }

        let suggestions = self.get_suggestions(input)?;
        if suggestions.is_empty() {
            return Ok(None);
        }
        if suggestions.len() == 1 {
            return Ok(Some(format!("{}/", suggestions[0])));
        }

        let lcp = longest_common_prefix(&suggestions);
        if lcp.len() > input.len() {
            Ok(Some(lcp))
        } else {
            Ok(None)
        }
    }
}

pub fn input_directory(page_size: usize, action: Action) -> Result<PathBuf> {
    let home = PathBuf::from(std::env::var("HOME").unwrap_or_default());
    let input = Text::new("Directory (TAB to complete):")
        .with_render_config(action.render_config())
        .with_autocomplete(DirCompleter::new(home.clone()))
        .with_page_size(page_size)
        .with_validator(|input: &str| {
            if input.trim().is_empty() {
                Ok(Validation::Invalid(
                    "Directory path cannot be empty.".into(),
                ))
            } else {
                Ok(Validation::Valid)
            }
        })
        .with_help_message("Type a path relative to ~ and press TAB to complete")
        .prompt()?;

    let input = input.trim_end_matches('/');
    Ok(home.join(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn available_actions_with_sessions_returns_all() {
        let actions = available_actions(true);
        assert_eq!(
            actions,
            vec![
                Action::CreateWithDir,
                Action::Create,
                Action::Attach,
                Action::Delete,
            ]
        );
    }

    #[test]
    fn available_actions_without_sessions_returns_create_and_create_with_dir() {
        let actions = available_actions(false);
        assert_eq!(actions, vec![Action::CreateWithDir, Action::Create]);
    }

    #[test]
    fn select_session_optional_returns_none_when_empty() {
        let result = select_session_optional(&[], Action::Delete).unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn longest_common_prefix_returns_shared_prefix() {
        let items = vec![
            "develop/foo".to_string(),
            "develop/bar".to_string(),
            "develop/baz".to_string(),
        ];
        assert_eq!(longest_common_prefix(&items), "develop/");
    }

    #[test]
    fn longest_common_prefix_returns_empty_when_no_common() {
        let items = vec!["alpha".to_string(), "beta".to_string()];
        assert_eq!(longest_common_prefix(&items), "");
    }

    #[test]
    fn longest_common_prefix_returns_item_for_single_element() {
        let items = vec!["only".to_string()];
        assert_eq!(longest_common_prefix(&items), "only");
    }

    #[test]
    fn longest_common_prefix_returns_empty_for_empty_input() {
        let items: Vec<String> = vec![];
        assert_eq!(longest_common_prefix(&items), "");
    }
}
