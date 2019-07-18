use gumdrop::Options;

#[derive(Debug, Clone, Options)]
pub struct Args {
    #[options(help = "display this message")]
    pub help: bool,

    #[options(help = "look up the dependencies for this crate instead")]
    pub deps: bool,

    #[options(help = "a specific version")]
    pub version: Option<String>,

    #[options(help = "list all versions")]
    pub list: bool, // TODO this should do nothing when --deps is used

    #[options(help = "shows any yanked versions before the latest stable")]
    pub show_yanked: bool, // TODO this should do nothing when --deps is used

    #[options(help = "prints results as json")]
    pub json: bool,

    #[options(help = "disables using colors when printing as text")]
    pub no_color: bool,

    #[options(help = "tries to use colors when printing as text")]
    pub color: bool,

    #[options(free)]
    pub name: String,
}
