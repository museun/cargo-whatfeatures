#[derive(Debug, Copy, Clone)]
pub struct Style {
    pub pipe: &'static str,
    pub branch: &'static str,
    pub edge: &'static str,
    pub right: &'static str,
    pub last: &'static str,
}

impl Default for Style {
    #[inline]
    fn default() -> Self {
        Self {
            pipe: "│ ",
            edge: "└─ ",
            branch: "├─ ",
            right: "─ ",
            last: "  ",
        }
    }
}
