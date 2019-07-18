use gumdrop::Options;

#[derive(Debug, Clone, Options)]
pub struct Args {
    #[options(help = "display this message")]
    pub help: bool,

    #[options(help = "look up the dependencies for this crate instead")]
    pub deps: bool,

    #[options(help = "a specific version")]
    pub version: Option<String>,

    #[options(help = "displays the features", meta = "bool", default = "true")]
    pub features: LexicalBool,

    #[options(help = "list only the name/version for the crate")]
    pub only_version: bool,

    #[options(help = "list all versions")]
    pub list: bool, // TODO this should do nothing when --deps is used

    #[options(help = "shows any yanked versions before the latest stable")]
    pub show_yanked: bool, // TODO this should do nothing when --deps is used

    #[options(help = "prints results as json")]
    pub json: bool,

    #[options(help = "disables using colors when printing as text")]
    pub no_color: bool,

    #[options(help = "tries to use colors when printing as text", default = "true")]
    pub color: bool,

    #[options(free)]
    pub name: String,
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct LexicalBool(bool);

impl std::ops::Deref for LexicalBool {
    type Target = bool;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq<bool> for LexicalBool {
    fn eq(&self, other: &bool) -> bool {
        *other == self.0
    }
}

impl std::str::FromStr for LexicalBool {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const TRUE: [&str; 4] = ["true", "t", "1", "yes"];
        const FALSE: [&str; 4] = ["false", "f", "0", "no"];

        match s.to_ascii_lowercase().as_str() {
            s if TRUE.contains(&s) => Ok(LexicalBool(true)),
            s if FALSE.contains(&s) => Ok(LexicalBool(false)),
            _ => Err(format!(
                "not a boolean: {}. only {} are allowed",
                s,
                TRUE.iter()
                    .zip(FALSE.iter())
                    .map(|(t, f)| format!("'{}' or '{}'", t, f))
                    .fold(String::new(), |mut a, b| {
                        if !a.is_empty() {
                            a.push_str(", ")
                        }
                        a.push_str(&b);
                        a
                    })
            )),
        }
    }
}
