use std::{cmp::Ordering, fmt::Display};

use super::{
    parser::DiffPackage,
    versioning::{Version, VersionComponent, VersionList},
};

#[derive(Default, Debug, PartialEq, Eq)]
pub enum DiffType {
    Added,
    Removed,
    Changed,

    #[default]
    Unknown,
}

#[derive(Debug)]
pub struct Package {
    pub size_delta: i64,
    pub diff_type: DiffType,

    pub versions_before: VersionList,
    pub versions_after: VersionList,
}

impl DiffType {
    pub fn from_versions(before: &[String], after: &[String]) -> DiffType {
        match (before.is_empty(), after.is_empty()) {
            (true, false) => DiffType::Added,
            (false, true) => DiffType::Removed,
            (false, false) => DiffType::Changed,
            (true, true) => DiffType::Unknown, // should be unreachable but I'm not sure
        }
    }
}

impl Display for Package {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.diff_type {
            DiffType::Added => {
                write!(f, "{}", self.versions_after)?;
            }
            DiffType::Removed => {
                write!(f, "{}", self.versions_before)?;
            }
            DiffType::Changed => {
                write!(f, "{} -> {}", self.versions_before, self.versions_after)?;
            }
            DiffType::Unknown => unreachable!(),
        }

        Ok(())
    }
}

impl From<DiffPackage> for Package {
    fn from(diff: DiffPackage) -> Self {
        let diff_type = DiffType::from_versions(&diff.versions_before, &diff.versions_after);

        let (parsed_before, parsed_after) = match diff_type {
            DiffType::Added => handle_diff_added(&diff.versions_after),
            DiffType::Removed => handle_diff_removed(&diff.versions_before),
            DiffType::Changed => handle_diff_changed(&diff.versions_before, &diff.versions_after),
            DiffType::Unknown => unreachable!(),
        };

        Package {
            size_delta: diff.size_delta,
            versions_before: parsed_before,
            versions_after: parsed_after,
            diff_type,
        }
    }
}

fn handle_diff_added(versions_after: &[String]) -> (VersionList, VersionList) {
    let mut parsed_after = VersionList::new();

    for after in versions_after {
        let parts_after = after.split('.').map(String::from);
        let mut version = Version::new();
        for part in parts_after {
            version.push(VersionComponent::new(part, Ordering::Greater));
        }
        parsed_after.push(version);
    }

    (VersionList::new(), parsed_after)
}

fn handle_diff_removed(versions_before: &[String]) -> (VersionList, VersionList) {
    let mut parsed_before = VersionList::new();
    for before in versions_before {
        let parts_before = before.split('.').map(String::from);
        let mut version = Version::new();
        for part in parts_before {
            version.push(VersionComponent::new(part, Ordering::Less));
        }
        parsed_before.push(version);
    }
    (parsed_before, VersionList::new())
}

fn handle_diff_changed(
    versions_before: &[String],
    versions_after: &[String],
) -> (VersionList, VersionList) {
    let mut parsed_before = VersionList::new();
    let mut parsed_after = VersionList::new();

    for (before, after) in versions_before.iter().zip(versions_after.iter()) {
        let mut parts_before = before.split('.').map(String::from).collect::<Vec<_>>();
        let mut parts_after = after.split('.').map(String::from).collect::<Vec<_>>();

        let max_len = parts_before.len().max(parts_after.len());
        parts_before.resize(max_len, String::new());
        parts_after.resize(max_len, String::new());

        let mut ordering = Ordering::Equal;

        let mut line_before = Version::new();
        let mut line_after = Version::new();

        for (b, a) in parts_before.into_iter().zip(parts_after.into_iter()) {
            if ordering == Ordering::Equal {
                ordering = b.cmp(&a);
            }
            line_before.push(VersionComponent::new(b, ordering));
            line_after.push(VersionComponent::new(a, ordering.reverse()));
        }

        parsed_before.push(line_before);
        parsed_after.push(line_after);
    }

    (parsed_before, parsed_after)
}
