use colored::{Color, ColoredString, Colorize, Styles};
use lazy_static::lazy_static;
use std::string::ToString;
use std::vec;

lazy_static! {
    pub static ref ERROR: ColoredString = "ERR:  ".red().bold();
    pub static ref WARN: ColoredString = "WARN: ".yellow().bold();
}

macro_rules! new_style {
    (right_align: $right_align:expr, $color:expr, $($style:expr),*) => {
        ComponentStyle {
            right_align: $right_align,
            color: $color,
            styles: vec![$($style),*],
        }
    };
    ($color:expr, $($style:expr),*) => {
        ComponentStyle {
            right_align: false,
            color: $color,
            styles: vec![$($style),*],
        }
    };
}

#[derive(Debug, Clone)]
pub struct ComponentStyle {
    pub right_align: bool,
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
    pub nesting: ComponentStyle,
    pub repo_size: ComponentStyle,
    pub remote_name: ComponentStyle,
    pub remote_fetch: ComponentStyle,
    pub remote_push: ComponentStyle,
    pub name: ComponentStyle,
    pub branch: ComponentStyle,
    pub detached: ComponentStyle,
    pub commit_hash: ComponentStyle,
    pub author_name: ComponentStyle,
    pub author_email: ComponentStyle,
    pub relative_time: ComponentStyle,
    pub absolute_time: ComponentStyle,
    pub dirty_style: ComponentStyle,
    pub bare_style: ComponentStyle,
    pub cs_author_count: ComponentStyle,
    pub cs_top_commit_count: ComponentStyle,
    pub cs_top_author_name: ComponentStyle,
    pub cs_top_author_email: ComponentStyle,
    pub top_lang: ComponentStyle,
}

impl Default for Palette {
    fn default() -> Self {
        Self {
            path: new_style!(Color::BrightBlue, Styles::Dimmed, Styles::Bold),
            nesting: new_style!(right_align: true, Color::BrightCyan, Styles::Dimmed, Styles::Bold),
            repo_size: new_style!(Color::BrightMagenta, Styles::Dimmed),
            remote_name: new_style!(Color::Green, Styles::Bold),
            remote_fetch: new_style!(Color::Blue, Styles::Clear),
            remote_push: new_style!(Color::Yellow, Styles::Dimmed),
            name: new_style!(Color::White, Styles::Bold),
            branch: new_style!(Color::Green, Styles::Clear),
            detached: new_style!(Color::Red, Styles::Italic),
            commit_hash: new_style!(Color::Cyan, Styles::Italic),
            author_name: new_style!(Color::BrightCyan, Styles::Clear),
            author_email: new_style!(Color::BrightBlack, Styles::Clear),
            relative_time: new_style!(right_align: true, Color::BrightRed, Styles::Clear),
            absolute_time: new_style!(Color::White, Styles::Clear),
            dirty_style: new_style!(Color::Red, Styles::Bold, Styles::Italic),
            bare_style: new_style!(Color::BrightRed, Styles::Bold, Styles::Italic, Styles::Dimmed),
            cs_author_count: new_style!(Color::Magenta, Styles::Clear),
            cs_top_commit_count: new_style!(Color::Yellow, Styles::Bold),
            cs_top_author_name: new_style!(Color::BrightCyan, Styles::Bold),
            cs_top_author_email: new_style!(Color::BrightWhite, Styles::Dimmed),
            top_lang: new_style!(Color::BrightYellow, Styles::Dimmed, Styles::Bold),
        }
    }
}
