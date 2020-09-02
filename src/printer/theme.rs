use yansi::Color;

#[derive(Copy, Clone)]
pub struct Theme {
    pub workspace: Color,
    pub name: Color,
    pub version: Color,
    pub yanked: Color,

    pub created_at: Color,
    pub license: Color,

    pub is_not_published: Color,

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

    pub required_deps: Color,
    pub optional_deps: Color,

    pub normal_deps: Color,

    pub dev_deps: Color,
    pub build_deps: Color,

    pub renamed: Color,
    pub renamed_target: Color,

    pub target: Color,
    pub dep_feature: Color,

    pub tree: Color,
}

impl std::fmt::Debug for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Theme").finish()
    }
}

impl Theme {
    /// Get a basic theme
    pub const fn basic() -> Self {
        BASIC_THEME
    }

    /// Get a colorful theme
    pub const fn colorful() -> Self {
        DEFAULT_THEME
    }
}

impl Default for Theme {
    fn default() -> Self {
        DEFAULT_THEME
    }
}

const DEFAULT_THEME: Theme = Theme {
    workspace: Color::RGB(255, 192, 192),

    name: Color::RGB(255, 192, 128),
    version: Color::RGB(192, 192, 0),
    yanked: Color::RGB(255, 0, 0),

    created_at: Color::RGB(255, 255, 255),
    license: Color::RGB(255, 255, 255),

    is_not_published: Color::RGB(0, 153, 238),

    no_default_features: Color::RGB(255, 128, 128),
    no_features: Color::RGB(192, 192, 192),
    no_optional_deps: Color::RGB(192, 128, 128),
    no_required_deps: Color::RGB(192, 192, 192),
    no_dev_deps: Color::RGB(192, 192, 192),
    no_build_deps: Color::RGB(192, 192, 192),

    has_enabled_features: Color::RGB(0, 153, 238),
    features: Color::RGB(192, 0, 192),
    feature_name: Color::RGB(255, 255, 255),
    feature_implies: Color::RGB(192, 192, 255),

    probably_internal: Color::RGB(128, 32, 32),

    default: Color::RGB(0, 255, 0),

    required_deps: Color::RGB(192, 0, 255),
    optional_deps: Color::RGB(255, 0, 192),

    normal_deps: Color::RGB(255, 124, 201),

    dev_deps: Color::RGB(255, 124, 201),
    build_deps: Color::RGB(255, 124, 201),

    renamed: Color::RGB(0, 153, 238),
    renamed_target: Color::RGB(92, 64, 255),

    target: Color::RGB(192, 0, 0),
    dep_feature: Color::RGB(128, 128, 128),

    tree: Color::RGB(48, 48, 48),
};

const BASIC_THEME: Theme = Theme {
    workspace: Color::RGB(255, 192, 192),

    name: Color::RGB(255, 192, 128),
    version: Color::Unset,
    yanked: Color::RGB(255, 0, 0),

    created_at: Color::Unset,
    license: Color::Unset,

    is_not_published: Color::Unset,

    no_default_features: Color::Unset,
    no_features: Color::Unset,
    no_optional_deps: Color::Unset,
    no_required_deps: Color::Unset,
    no_dev_deps: Color::Unset,
    no_build_deps: Color::Unset,

    has_enabled_features: Color::Unset,
    features: Color::Unset,
    feature_name: Color::Unset,
    feature_implies: Color::RGB(192, 192, 255),

    probably_internal: Color::Unset,

    default: Color::RGB(0, 255, 0),

    required_deps: Color::Unset,
    optional_deps: Color::Unset,

    normal_deps: Color::Unset,

    dev_deps: Color::Unset,
    build_deps: Color::Unset,

    renamed: Color::Unset,
    renamed_target: Color::Unset,

    target: Color::Unset,
    dep_feature: Color::Unset,

    tree: Color::RGB(48, 48, 48),
};
