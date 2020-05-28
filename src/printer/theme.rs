use yansi::Color;

pub struct Theme {
    pub name: Color,
    pub version: Color,
    pub yanked: Color,

    pub no_default_features: Color,
    pub no_features: Color,
    pub no_optional_deps: Color,
    pub no_required_deps: Color,
    pub no_dev_deps: Color,
    pub no_build_deps: Color,

    pub has_enabled_features: Color,
    pub features: Color,
    pub feature_name: Color,
    pub feature_implies: Color,

    pub probably_internal: Color,

    pub default: Color,
    pub default_features: Color,

    pub required_deps: Color,
    pub optional_deps: Color,

    pub normal_deps: Color,

    pub dev_deps: Color,
    pub build_deps: Color,

    pub renamed: Color,
    pub target: Color,
    pub dep_feature: Color,

    pub tree: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            name: Color::RGB(255, 192, 128),
            version: Color::RGB(192, 192, 0),
            yanked: Color::RGB(255, 0, 0),

            no_default_features: Color::RGB(255, 170, 0),
            no_features: Color::RGB(255, 170, 0),
            no_optional_deps: Color::RGB(255, 170, 0),
            no_required_deps: Color::RGB(255, 170, 0),
            no_dev_deps: Color::RGB(255, 170, 0),
            no_build_deps: Color::RGB(255, 170, 0),

            has_enabled_features: Color::RGB(0, 153, 238),
            features: Color::RGB(255, 0, 255),
            feature_name: Color::RGB(255, 255, 255),
            feature_implies: Color::RGB(192, 192, 255),

            probably_internal: Color::RGB(128, 32, 32),

            default: Color::RGB(0, 255, 0),
            default_features: Color::RGB(0, 192, 0),

            required_deps: Color::RGB(192, 0, 255),
            optional_deps: Color::RGB(255, 0, 255),

            normal_deps: Color::RGB(255, 124, 201),

            dev_deps: Color::RGB(255, 124, 201),
            build_deps: Color::RGB(255, 124, 201),

            renamed: Color::RGB(64, 0, 255),
            target: Color::RGB(192, 0, 0),
            dep_feature: Color::RGB(128, 128, 128),

            tree: Color::RGB(48, 48, 48),
        }
    }
}
