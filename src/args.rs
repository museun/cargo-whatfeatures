use gumdrop::Options;

#[derive(Debug, Clone, Options)]
pub struct Args {
    #[options(help = "display this message")]
    pub help: bool,

    #[options(help = "a specific version")]
    pub version: Option<String>,

    #[options(help = "list all versions")]
    pub list: bool,

    #[options(help = "prints results as json")]
    pub json: bool,

    #[options(help = "shows any yanked versions before the latest stable")]
    pub show_yanked: bool,

    #[options(help = "disables using colors when printing as text")]
    pub no_color: bool,

    #[options(help = "tries to use colors when printing as text")]
    pub color: bool,

    #[options(help = "look up the depencies for this crate instead")]
    pub deps: bool,

    #[options(free)]
    pub name: String,
}
