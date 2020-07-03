mod deps;
mod labels;
mod tree;

mod style;
use style::Style;

mod theme;
pub use theme::Theme;

mod yank_status;
pub use yank_status::YankStatus;

mod workspace;
pub use workspace::WorkspacePrinter;

mod version;
pub use version::VersionPrinter;

#[derive(Copy, Clone)]
pub struct Options {
    pub print_features: bool,
    pub show_private: bool,
    pub show_deps: bool,
    pub verbose: bool,
}
