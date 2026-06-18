use colored::{Color, Colorize, Styles};
use std::vec;

macro_rules! new_style {
    ($color:expr, $($style:expr),*) => {
        ComponentStyle {
            color: $color,
            styles: vec![$($style),*],
        }
    };
}

#[derive(Debug, Clone)]
pub struct ComponentStyle {
    pub color: Color,
    pub styles: Vec<Styles>,
}

impl ComponentStyle {
    pub fn apply(&self, text: &str) -> String {
        let mut colored = text.color(self.color);
        self.styles.iter().for_each(|style| colored.style.add(*style));
        colored.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct Palette {
    pub path: ComponentStyle,
    pub repo_size: ComponentStyle,
    pub name: ComponentStyle,
    pub branch: ComponentStyle,
    pub detached: ComponentStyle,
    pub commit_hash: ComponentStyle,
    pub author_name: ComponentStyle,
    pub author_email: ComponentStyle,
    pub relative_time: ComponentStyle,
    pub absolute_time: ComponentStyle,
    pub dirty_style: ComponentStyle,
    pub clean_style: ComponentStyle,
    pub bare_style: ComponentStyle,
    pub cs_author_count: ComponentStyle,
    pub cs_top_commit_count: ComponentStyle,
    pub cs_top_author_name: ComponentStyle,
    pub cs_top_author_email: ComponentStyle,
}

impl Default for Palette {
    fn default() -> Self {
        Self {
            path: new_style!(Color::BrightBlue, Styles::Dimmed, Styles::Bold),
            repo_size: new_style!(Color::BrightMagenta, Styles::Dimmed),
            name: new_style!(Color::White, Styles::Bold),
            branch: new_style!(Color::Green, Styles::Clear),
            detached: new_style!(Color::Red, Styles::Italic),
            commit_hash: new_style!(Color::Cyan, Styles::Italic),
            author_name: new_style!(Color::BrightCyan, Styles::Clear),
            author_email: new_style!(Color::BrightBlack, Styles::Clear),
            relative_time: new_style!(Color::BrightRed, Styles::Clear),
            absolute_time: new_style!(Color::White, Styles::Clear),
            dirty_style: new_style!(Color::Red, Styles::Bold),
            bare_style: new_style!(Color::Red, Styles::Italic),
            clean_style: new_style!(Color::Green, Styles::Clear),
            cs_author_count: new_style!(Color::Magenta, Styles::Clear),
            cs_top_commit_count: new_style!(Color::Yellow, Styles::Bold),
            cs_top_author_name: new_style!(Color::BrightCyan, Styles::Bold),
            cs_top_author_email: new_style!(Color::BrightWhite, Styles::Dimmed),
        }
    }
}
