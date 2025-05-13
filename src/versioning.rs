use nu_ansi_term::Color::{Green, Red, Yellow};
use std::{cmp::Ordering, fmt::Display};

#[derive(Debug, Clone)]
pub struct VersionComponent(String, Ordering);

#[derive(Debug, Clone)]
pub struct Version(Vec<VersionComponent>);

#[derive(Debug, Clone)]
pub struct VersionList(pub Vec<Version>);

impl VersionComponent {
    pub fn new(version: String, ordering: Ordering) -> Self {
        Self(version, ordering)
    }
}

impl Version {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, version: VersionComponent) {
        self.0.push(version);
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();

        for component in &self.0 {
            let val = &component.0;
            let cmp = component.1;

            let text = if cmp == Ordering::Less {
                format!("{}", Red.paint(val))
            } else if cmp == Ordering::Greater {
                format!("{}", Green.paint(val))
            } else {
                format!("{}", Yellow.paint(val))
            };

            out.push_str(&text);
            out.push('.');
        }

        out.pop(); // remove last comma
        write!(f, "{out}")
    }
}

impl VersionList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, version: Version) {
        self.0.push(version);
    }
}

impl Display for VersionList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        for version in &self.0 {
            out.push_str(&version.to_string());
            out.push_str(", ");
        }
        out.pop(); // remove last comma
        out.pop(); // remove last space
        write!(f, "{out}")
    }
}
