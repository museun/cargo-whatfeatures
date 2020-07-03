use crate::features::{Dependency, Kind};
use std::collections::{BTreeMap, HashMap};

#[derive(Debug)]
pub struct SortedDeps {
    pub normal: GroupedDeps,
    pub development: GroupedDeps,
    pub build: GroupedDeps,
}

impl SortedDeps {
    // we should have a workspace of map<crate_name, hashmap<kind, vec<dep>>>
    pub fn from_kind_map(mut map: HashMap<Kind, Vec<Dependency>>) -> Self {
        // this is accidently flattening the tree
        fn categorize(kind: Kind, kind_map: &mut HashMap<Kind, Vec<Dependency>>) -> GroupedDeps {
            let (mut other, mut map) = (Vec::new(), BTreeMap::new());
            if let Some(deps) = kind_map.remove(&kind) {
                for dep in deps {
                    if let Some(target) = &dep.target {
                        map.entry(target.clone()).or_default()
                    } else {
                        &mut other
                    }
                    .push(dep);
                }
                for val in map.values_mut() {
                    val.sort_by(|left, right| left.name.cmp(&right.name));
                }
                other.sort_by(|left, right| left.name.cmp(&right.name));
            }

            GroupedDeps {
                with_targets: map,
                without_targets: other,
            }
        }

        let normal = categorize(Kind::Normal, &mut map);
        let development = categorize(Kind::Development, &mut map);
        let build = categorize(Kind::Build, &mut map);
        assert!(map.is_empty());

        Self {
            normal,
            development,
            build,
        }
    }
}

#[derive(Debug)]
pub struct GroupedDeps {
    pub with_targets: BTreeMap<String, Vec<Dependency>>,
    pub without_targets: Vec<Dependency>,
}

impl GroupedDeps {
    pub fn has_deps(&self) -> bool {
        !(self.with_targets.is_empty() && self.without_targets.is_empty())
    }
}
