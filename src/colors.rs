use colored::Color;

pub struct Colors {
    pub path: Color,
    pub name: Color,
    pub branch: Color,
    pub commit_hash: Color,
    pub author_name: Color,
    pub author_email: Color,
    pub relative_time: Color,
    pub absolute_time: Color,
}

impl Default for Colors {
    fn default() -> Self {
        Self {
            path: Color::TrueColor { r: 129, g: 161, b: 193 },
            name: Color::TrueColor { r: 216, g: 222, b: 233 },
            branch: Color::TrueColor { r: 163, g: 190, b: 140 },
            commit_hash: Color::TrueColor { r: 143, g: 188, b: 187 },
            author_name: Color::TrueColor { r: 136, g: 192, b: 208 },
            author_email: Color::TrueColor { r: 76, g: 86, b: 106 },
            relative_time: Color::TrueColor { r: 208, g: 135, b: 112 },
            absolute_time: Color::TrueColor { r: 140, g: 153, b: 165 },
        }
    }
}
