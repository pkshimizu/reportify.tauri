use super::Theme;

#[derive(Debug, Clone)]
pub struct Settings {
    pub theme: Theme,
    pub language: String,
}

impl Settings {
    pub fn new(theme: Theme, language: String) -> Self {
        Self { theme, language }
    }
}
