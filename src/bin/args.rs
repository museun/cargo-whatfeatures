use lexical_bool::LexicalBool;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct Args {
    /// A specific version to lookup. e.g. 0.7.1
    #[structopt(short, long, value_name = "semver")]
    pub version: Option<String>,

    /// Display only the name and version, such as foo/0.1.2
    #[structopt(short, long, conflicts_with = "deps")]
    pub short: bool,

    /// List all versions for the crate
    #[structopt(short, long, conflicts_with = "version")]
    pub list: bool,

    /// Shows any yanked versions. Defaults to hiding them
    #[structopt(short = "y", long)]
    pub show_yanked: bool,

    /// Attempts to use colors when printing as text
    #[structopt(
        short,
        long,
        parse(try_from_str),
        value_name = "bool",
        default_value = "true"
    )]
    pub color: LexicalBool,

    /// Use JSON as the output format. Defaults to a textual format
    #[structopt(short, long)]
    pub json: bool,

    /// The name of the crate to retrieve information for
    #[structopt(name = "crate")]
    pub name: String,

    /// Disable listing the features for the crate
    #[structopt(short, long, requires = "deps")]
    pub no_features: bool,

    /// Display dependencies for this crate
    #[structopt(short, long)]
    pub deps: bool,
}
