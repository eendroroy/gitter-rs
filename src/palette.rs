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
    pub dirty_style: ComponentStyle,
    pub clean_style: ComponentStyle,
    pub cs_commit_count: ComponentStyle,
    pub cs_top_commit_count: ComponentStyle,
    pub cs_top_author_name: ComponentStyle,
    pub cs_top_author_email: ComponentStyle,
}

impl Default for Palette {
    fn default() -> Self {
        Self {
            path: ComponentStyle {
                color: Color::TrueColor { r: 129, g: 161, b: 193 }, // Nord 9 (Nordic Blue)
                styles: vec![Styles::Dimmed, Styles::Bold, Styles::Italic],
            },
            name: ComponentStyle {
                color: Color::TrueColor { r: 216, g: 222, b: 233 }, // Nord 4 (Snow Storm)
                styles: vec![Styles::Bold],
            },
            branch: ComponentStyle {
                color: Color::TrueColor { r: 163, g: 190, b: 140 }, // Nord 14 (Green)
                styles: vec![Styles::Clear],
            },
            commit_hash: ComponentStyle {
                color: Color::TrueColor { r: 143, g: 188, b: 187 }, // Nord 7 (Aurora Teal)
                styles: vec![Styles::Italic],
            },
            author_name: ComponentStyle {
                color: Color::TrueColor { r: 136, g: 192, b: 208 }, // Nord 8 (Ice Blue)
                styles: vec![Styles::Clear],
            },
            author_email: ComponentStyle {
                color: Color::TrueColor { r: 76, g: 86, b: 106 }, // Nord 3 (Polar Night Gray)
                styles: vec![Styles::Clear],
            },
            relative_time: ComponentStyle {
                color: Color::TrueColor { r: 208, g: 135, b: 112 }, // Nord 12 (Orange)
                styles: vec![Styles::Clear],
            },
            absolute_time: ComponentStyle {
                color: Color::TrueColor { r: 140, g: 153, b: 165 }, // Custom Gray/Blue
                styles: vec![Styles::Clear],
            },
            dirty_style: ComponentStyle {
                color: Color::TrueColor { r: 166, g: 74, b: 82 }, // Muted Nord Red (Dimmed down)
                styles: vec![Styles::Bold],                       // Kept Bold for clear visibility
            },
            clean_style: ComponentStyle {
                color: Color::TrueColor { r: 163, g: 190, b: 140 }, // Nord 14 (Green - matches branch)
                styles: vec![Styles::Clear],
            },
            cs_commit_count: ComponentStyle {
                color: Color::TrueColor { r: 180, g: 142, b: 173 }, // Nord 15 (Purple)
                styles: vec![Styles::Clear],
            },
            cs_top_commit_count: ComponentStyle {
                color: Color::TrueColor { r: 235, g: 203, b: 139 }, // Nord 13 (Yellow/Gold) to highlight top stat
                styles: vec![Styles::Bold],
            },
            cs_top_author_name: ComponentStyle {
                color: Color::TrueColor { r: 136, g: 192, b: 208 }, // Nord 8 (Matching standard author name)
                styles: vec![Styles::Bold],                         // Bold to signify "Top" status
            },
            cs_top_author_email: ComponentStyle {
                color: Color::TrueColor { r: 229, g: 233, b: 240 }, // Nord 5 (Snow Storm Light Gray)
                styles: vec![Styles::Dimmed],                       // Dimmed to stay secondary
            },
        }
    }
}
