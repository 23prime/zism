use inquire::ui::{Attributes, Color, RenderConfig, StyleSheet};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Create,
    CreateWithDir,
    Attach,
    Delete,
}

impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::Create => write!(f, "Create new session"),
            Action::CreateWithDir => write!(f, "Create new session with directory"),
            Action::Attach => write!(f, "Attach to session"),
            Action::Delete => write!(f, "Delete session"),
        }
    }
}

impl Action {
    fn color(&self) -> Color {
        match self {
            Action::Create | Action::CreateWithDir => Color::LightGreen,
            Action::Attach => Color::LightCyan,
            Action::Delete => Color::LightRed,
        }
    }

    fn highlight_color(&self) -> Color {
        match self {
            Action::Create | Action::CreateWithDir => Color::DarkGreen,
            Action::Attach => Color::DarkCyan,
            Action::Delete => Color::DarkRed,
        }
    }

    pub fn render_config(&self) -> RenderConfig<'static> {
        RenderConfig {
            prompt: StyleSheet::new()
                .with_attr(Attributes::BOLD)
                .with_fg(self.color()),
            selected_option: Some(
                StyleSheet::new()
                    .with_fg(Color::Black)
                    .with_bg(self.highlight_color()),
            ),
            ..RenderConfig::default()
        }
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
    fn action_display_create_with_dir() {
        assert_eq!(
            Action::CreateWithDir.to_string(),
            "Create new session with directory"
        );
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
    fn action_color_returns_light_green_for_create() {
        assert_eq!(Action::Create.color(), Color::LightGreen);
    }

    #[test]
    fn action_color_returns_light_green_for_create_with_dir() {
        assert_eq!(Action::CreateWithDir.color(), Color::LightGreen);
    }

    #[test]
    fn action_color_returns_light_cyan_for_attach() {
        assert_eq!(Action::Attach.color(), Color::LightCyan);
    }

    #[test]
    fn action_color_returns_light_red_for_delete() {
        assert_eq!(Action::Delete.color(), Color::LightRed);
    }

    #[test]
    fn action_highlight_color_returns_dark_green_for_create() {
        assert_eq!(Action::Create.highlight_color(), Color::DarkGreen);
    }

    #[test]
    fn action_highlight_color_returns_dark_green_for_create_with_dir() {
        assert_eq!(Action::CreateWithDir.highlight_color(), Color::DarkGreen);
    }

    #[test]
    fn action_highlight_color_returns_dark_cyan_for_attach() {
        assert_eq!(Action::Attach.highlight_color(), Color::DarkCyan);
    }

    #[test]
    fn action_highlight_color_returns_dark_red_for_delete() {
        assert_eq!(Action::Delete.highlight_color(), Color::DarkRed);
    }
}
