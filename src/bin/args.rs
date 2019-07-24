use gumdrop::Options;
use lexical_bool::LexicalBool;

#[derive(Debug, Clone, Options)]
pub struct Args {
    #[options(help = "Displays this help message")]
    pub help: bool,

    #[options(help = "Display dependencies for this crate")]
    pub deps: bool,

    #[options(help = "A specific version to lookup. e.g. 0.7.1", meta = "SEMVER")]
    pub version: Option<String>,

    #[options(
        help = "Display the features for the crate",
        meta = "bool",
        default = "true"
    )]
    pub features: LexicalBool,

    #[options(help = "Only list the name and version, rather than extended info")]
    pub short: bool,

    #[options(help = "List all versions for the crate")]
    pub list: bool,

    #[options(
        help = "Shows any yanked versions. Defaults to hiding them",
        short = "y"
    )]
    pub show_yanked: bool,

    #[options(
        help = "Attempts to use colors when printing as text",
        meta = "bool",
        default = "true"
    )]
    pub color: LexicalBool,

    #[options(help = "Use JSON as the output format. Defaults to a textual format")]
    pub json: bool,

    #[options(help = "The name of the crate to retrieve information for")]
    #[options(free)]
    pub name: String,
}
