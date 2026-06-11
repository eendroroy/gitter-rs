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
            path: Color::BrightBlack,
            name: Color::BrightBlue,
            branch: Color::Green,
            commit_hash: Color::Cyan,
            author_name: Color::BrightGreen,
            author_email: Color::BrightGreen,
            relative_time: Color::Yellow,
            absolute_time: Color::Yellow,
        }
    }
}
