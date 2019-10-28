mod json;
mod text;

#[macro_use]
mod ser;

use std::collections::HashMap;

use crate::crates::{Dependency, DependencyKind, Version};

pub(crate) use self::{
    json::{Format, RenderAsJson},
    text::RenderAsText,
};

pub(crate) type Dependencies = HashMap<DependencyKind, Vec<Dependency>>;

/// A simple model consisting of a `name`, a `version` and a `yanked` flag
#[derive(serde::Serialize, Debug)]
pub struct SimpleModel<'a> {
    /// Name
    pub name: &'a str,
    /// Version
    pub version: &'a str,
    /// Yanked
    pub yanked: bool,
}

impl<'a> SimpleModel<'a> {
    /// Create a SimpleModel from a Version
    #[inline]
    pub fn from_version(version: &'a Version) -> Self {
        Self {
            name: &version.crate_,
            version: &version.num,
            yanked: version.yanked,
        }
    }
}

/// A model which contains a [`SimpleModel`](./struct.SimpleModel.html) and a [`crates::Version`](../crates/struct.Version.html)
#[derive(Debug)]
pub struct FeaturesModel<'a> {
    /// simple
    pub simple: SimpleModel<'a>,
    /// version
    pub version: &'a Version,
}

impl<'a> FeaturesModel<'a> {
    /// Create a FeaturesModel from a Version
    #[inline]
    pub fn from_version(version: &'a Version) -> Self {
        Self {
            simple: SimpleModel::from_version(&version),
            version,
        }
    }
}

/// A model which contains a [`SimpleModel`](./struct.SimpleModel.html) and a dependency mapping
#[derive(Debug)]
pub struct DependencyModel<'a> {
    /// Simple
    pub simple: SimpleModel<'a>,
    /// Dependencies
    pub dependencies: Dependencies,
}

impl<'a> DependencyModel<'a> {
    /// Create a DependencyModel from a Version and an iterator of Dependency
    pub fn from_vers_deps(
        version: &'a Version,
        deps: impl IntoIterator<Item = Dependency>,
    ) -> Self {
        Self {
            simple: SimpleModel::from_version(&version),
            dependencies: deps
                .into_iter()
                .fold(Dependencies::default(), |mut map, dep| {
                    map.entry(dep.kind).or_default().push(dep);
                    map
                }),
        }
    }
}

/// A model which contains a [`SimpleModel`](./struct.SimpleModel.html), a [`FeaturesModel`](./struct.FeaturesModel.html) and a [`DependencyModel`](./struct.DependencyModel.html)
#[derive(Debug)]
pub struct CompositeModel<'a> {
    /// Simple
    pub simple: SimpleModel<'a>,
    /// Features
    pub features: FeaturesModel<'a>,
    /// Dependencies
    pub dependencies: DependencyModel<'a>,
}

impl<'a> CompositeModel<'a> {
    /// Create a CompositeModel from a Version and an iterator of Dependency
    pub fn from_vers_deps(
        version: &'a Version,
        deps: impl IntoIterator<Item = Dependency>,
    ) -> Self {
        Self {
            simple: SimpleModel::from_version(&version),
            features: FeaturesModel::from_version(&version),
            dependencies: DependencyModel::from_vers_deps(version, deps),
        }
    }
}

/// A list of [`SimpleModels`](./struct.SimpleModel.html)
#[derive(Debug)]
pub struct SimpleListModel<'a> {
    /// Simple list
    pub simple_list: Vec<SimpleModel<'a>>,
}

impl<'a> SimpleListModel<'a> {
    #[inline]
    /// Create a new SimpleListModel from a collection of SimpleModel
    pub fn new(list: Vec<SimpleModel<'a>>) -> Self {
        Self { simple_list: list }
    }
}

impl<'a> std::iter::FromIterator<SimpleModel<'a>> for SimpleListModel<'a> {
    fn from_iter<T: IntoIterator<Item = SimpleModel<'a>>>(iter: T) -> Self {
        Self {
            simple_list: iter.into_iter().collect(),
        }
    }
}

/// A list of [`FeaturesModels`](./struct.FeaturesModel.html)
#[derive(Debug)]
pub struct FeaturesListModel<'a> {
    /// Features list
    pub features_list: Vec<FeaturesModel<'a>>,
}

impl<'a> FeaturesListModel<'a> {
    #[inline]
    /// Create a new FeaturesListModel from a collection of FeaturesModel
    pub fn new(list: Vec<FeaturesModel<'a>>) -> Self {
        Self {
            features_list: list,
        }
    }
}

impl<'a> std::iter::FromIterator<FeaturesModel<'a>> for FeaturesListModel<'a> {
    fn from_iter<T: IntoIterator<Item = FeaturesModel<'a>>>(iter: T) -> Self {
        Self {
            features_list: iter.into_iter().collect(),
        }
    }
}

/// Output allows for writing different formats to a Writer
#[derive(Debug)]
pub struct Output<'a, W> {
    writer: &'a mut W,
}

impl<'a, W: std::io::Write> Output<'a, W> {
    /// Create a new Output from a Writer
    pub fn new(writer: &'a mut W) -> Self {
        Self { writer }
    }

    /// Write json to the writer
    pub fn write_json(&mut self, data: &impl serde::Serialize) {
        data.render(self.writer, Format::Compact).unwrap();
    }

    /// Write text to the writer
    pub fn write_text(&mut self, data: &impl RenderAsText) {
        data.render(self.writer).unwrap()
    }
}

/// Render this model as json, using the provided writer
pub fn render_as_json(
    item: &impl serde::Serialize,
    writer: &mut impl std::io::Write,
) -> std::io::Result<()> {
    RenderAsJson::render(item, writer, Format::Compact)
}

/// Render this model as text, using the provided writer
pub fn render_as_text(
    item: &impl RenderAsText,
    writer: &mut impl std::io::Write,
) -> std::io::Result<()> {
    RenderAsText::render(item, writer)
}
