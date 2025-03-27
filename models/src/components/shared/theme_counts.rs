use super::{location::Location, theme::Theme};

pub struct ThemeCount {
    pub count_type: Theme,
    pub count: u128,
    pub object_type: Option<String>,
    pub location: Location,
    pub offset: Option<u128>,
}

pub struct ThemeCounts {
    pub theme_counts: Vec<ThemeCount>,
    pub themes: Vec<Theme>,
}
