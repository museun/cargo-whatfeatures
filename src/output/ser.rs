use super::*;
use serde::ser::*;

#[macro_export]
macro_rules! map {
    (@one $($x:tt)*) => (());
    (@len $($args:expr),*) => (<[()]>::len(&[$(map!(@one $args)),*]));
    ($ser:expr => $($k:expr => $v:expr,)+) => { map!($ser => $($k => $v),+) };
    ($ser:expr => $($k:expr => $v:expr),*) => {{
        let mut map = $ser.serialize_map(Some(map!(@len $($k),*)))?;
        $(
            map.serialize_key(&$k)?;
            map.serialize_value(&$v)?;
        )*
        map.end()
    }};
}

pub struct KeyVal<'a, T> {
    key: &'a str,
    val: T,
}

impl<'a, T: Serialize> KeyVal<'a, T> {
    pub fn new(key: &'a str, val: T) -> Self {
        Self { key, val }
    }
}

impl<'a, T: Serialize> Serialize for KeyVal<'a, T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_key(&self.key)?;
        map.serialize_value(&self.val)?;
        map.end()
    }
}

pub struct Map<I, F> {
    map: I,
    apply: F,
}

impl<K, V, T, U, F, I> Map<I, F>
where
    K: Serialize,
    V: Serialize,
    F: Fn(&T, &U) -> (K, V),
    I: Iterator<Item = (T, U)>,
{
    pub fn new(map: I, apply: F) -> Self {
        Self { map, apply }
    }
}

impl<K, V, T, U, F, I> Serialize for Map<I, F>
where
    K: Serialize,
    V: Serialize,
    F: Fn(&T, &U) -> (K, V),
    I: Iterator<Item = (T, U)> + Clone,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(1))?;
        for (k, v) in self.map.clone() {
            let (key, val) = (self.apply)(&k, &v);
            map.serialize_key(&key)?;
            map.serialize_value(&val)?;
        }
        map.end()
    }
}

pub struct List<'a, T, F> {
    list: &'a [T],
    apply: F,
}

impl<'a, S: Serialize, T, F: Fn(&'a T) -> S> List<'a, T, F> {
    pub fn new(list: &'a [T], apply: F) -> Self {
        Self { list, apply }
    }
}

impl<'a, S: Serialize, T, F: Fn(&'a T) -> S> Serialize for List<'a, T, F> {
    fn serialize<Ser>(&self, serializer: Ser) -> Result<Ser::Ok, Ser::Error>
    where
        Ser: Serializer,
    {
        let mut list = serializer.serialize_seq(Some(self.list.len()))?;
        for el in self.list {
            list.serialize_element(&(self.apply)(&el))?;
        }
        list.end()
    }
}

#[derive(serde::Serialize)]
struct Dep<'a> {
    req: &'a str,
    optional: bool,
    default_features: bool,
    features: &'a [String],
    target: Option<&'a String>,
}

impl<'a> Dep<'a> {
    fn from_dep(dep: &'a Dependency) -> Self {
        Self {
            req: &dep.req,
            optional: dep.optional,
            default_features: dep.default_features,
            features: &dep.features,
            target: dep.target.as_ref(),
        }
    }
}

impl<'a> Serialize for FeaturesModel<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        map! { serializer =>
            "name" => &self.simple.name,
            "version" => &self.simple.version,
            "yanked" => &self.simple.yanked,
            "features" => &self.version.features,
        }
    }
}

impl<'a> Serialize for DependencyModel<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        map! { serializer =>
            "name" => &self.simple.name,
            "version" => &self.simple.version,
            "yanked" => &self.simple.yanked,
            "dependencies" => &Map::new(
                self.dependencies.iter(), |t, u| (
                    match t {
                        DependencyKind::Normal => "normal",
                        DependencyKind::Dev => "dev",
                        DependencyKind::Build => "build",
                    },
                    List::new(&u, |inner| KeyVal::new(
                        &inner.crate_id, Dep::from_dep(&inner)
                    )
                ))
            )
        }
    }
}

impl<'a> Serialize for CompositeModel<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        map! { serializer =>
            "name" => &self.simple.name,
            "version" => &self.simple.version,
            "yanked" => &self.simple.yanked,
            "features" => &self.features.version.features,
            "dependencies" => &Map::new(
                self.dependencies.dependencies.iter(), |t, u| (
                    match t {
                        DependencyKind::Normal => "normal",
                        DependencyKind::Dev => "dev",
                        DependencyKind::Build => "build",
                    },
                    List::new(&u, |inner| KeyVal::new(
                        &inner.crate_id, Dep::from_dep(&inner)
                    )
                ))
            )
        }
    }
}

impl<'a> Serialize for SimpleListModel<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(serde::Serialize)]
        struct Inner<'a> {
            version: &'a str,
            yanked: bool,
        }

        KeyVal::new(
            &self
                .simple_list
                .get(0)
                .ok_or_else(|| Error::custom("features_list is empty"))?
                .name,
            List::new(&self.simple_list, |el| Inner {
                version: &el.version,
                yanked: el.yanked,
            }),
        )
        .serialize(serializer)
    }
}

impl<'a> Serialize for FeaturesListModel<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(serde::Serialize)]
        struct Inner<'a> {
            version: &'a str,
            features: &'a HashMap<String, Vec<String>>,
            yanked: bool,
        }

        let base = self
            .features_list
            .get(0)
            .ok_or_else(|| Error::custom("features_list is empty"))?;

        KeyVal::new(
            &base.simple.name,
            List::new(&self.features_list, |el| Inner {
                version: &el.version.num,
                features: &el.version.features,
                yanked: el.version.yanked,
            }),
        )
        .serialize(serializer)
    }
}
