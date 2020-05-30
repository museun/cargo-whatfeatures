/// When to show yanked crates
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum YankStatus {
    /// Only show yanked crates
    Only,
    /// Exclude all yanked crates
    Exclude,
    /// Include yanked crates
    Include,
}

impl Default for YankStatus {
    fn default() -> Self {
        Self::Exclude
    }
}
