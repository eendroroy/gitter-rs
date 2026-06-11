use colored::{Color, Colorize, Styles};
use std::vec;

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
    pub name: ComponentStyle,
    pub branch: ComponentStyle,
    pub commit_hash: ComponentStyle,
    pub author_name: ComponentStyle,
    pub author_email: ComponentStyle,
    pub relative_time: ComponentStyle,
    pub absolute_time: ComponentStyle,
}

impl Default for Palette {
    fn default() -> Self {
        Self {
            path: ComponentStyle {
                color: Color::TrueColor { r: 129, g: 161, b: 193 },
                styles: vec![Styles::Underline],
            },
            name: ComponentStyle {
                color: Color::TrueColor { r: 216, g: 222, b: 233 },
                styles: vec![Styles::Bold],
            },
            branch: ComponentStyle {
                color: Color::TrueColor { r: 163, g: 190, b: 140 },
                styles: vec![Styles::Clear],
            },
            commit_hash: ComponentStyle {
                color: Color::TrueColor { r: 143, g: 188, b: 187 },
                styles: vec![Styles::Italic],
            },
            author_name: ComponentStyle {
                color: Color::TrueColor { r: 136, g: 192, b: 208 },
                styles: vec![Styles::Clear],
            },
            author_email: ComponentStyle {
                color: Color::TrueColor { r: 76, g: 86, b: 106 },
                styles: vec![Styles::Clear],
            },
            relative_time: ComponentStyle {
                color: Color::TrueColor { r: 208, g: 135, b: 112 },
                styles: vec![Styles::Clear],
            },
            absolute_time: ComponentStyle {
                color: Color::TrueColor { r: 140, g: 153, b: 165 },
                styles: vec![Styles::Clear],
            },
        }
    }
}
