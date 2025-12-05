use nu_ansi_term::Color::{self, Green, Red, Yellow};
use std::collections::BTreeMap;

use super::{
    package::{DiffType, Package, SizeDelta},
    parser::DiffRoot,
};

#[derive(Debug)]
pub struct PackageExtra {
    name: String,
    base_package: Package,
}

#[derive(Debug)]
pub struct PackageListDiff {
    all: Vec<PackageExtra>,
    added: BTreeMap<String, Package>,
    removed: BTreeMap<String, Package>,
    changed: BTreeMap<String, Package>,
    size_delta: SizeDelta,
    longest_name: usize,

    // Whether to sort by size difference when displaying
    pub by_size: bool,
}

impl PackageListDiff {
    pub fn new() -> Self {
        PackageListDiff {
            all: Vec::new(),
            added: BTreeMap::new(),
            removed: BTreeMap::new(),
            changed: BTreeMap::new(),
            size_delta: SizeDelta(0),
            longest_name: 0,
            by_size: false,
        }
    }
}

impl std::fmt::Display for PackageListDiff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.by_size {
            self.display_by_size(f)?;
        } else {
            if self.added.is_empty() && self.removed.is_empty() && self.changed.is_empty() {
                return write!(f, "No differences found.");
            }

            self.display_by_category(f)?;
        }

        {
            let delta = &self.size_delta;
            writeln!(f, "size diff: {delta}")?;
        }

        Ok(())
    }
}

impl PackageListDiff {
    fn display_by_size(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let all = {
            let mut v: Vec<_> = self.all.iter().collect();
            v.sort_by_key(|pkg| -pkg.base_package.size_delta.0);
            v
        };

        let name_width = self.longest_name + 2;
        let package_width = (all
            .iter()
            .map(|pkg| strip_ansi_escapes::strip(format!("{}", pkg.base_package)).len())
            .max()
            .expect("At least one package exists"))
        .min(120);

        for package in &all {
            let name = &package.name;
            let package = &package.base_package;
            let delta = &package.size_delta;
            let versions = format!("{package}");
            let visual_len = strip_ansi_escapes::strip(&versions).len();
            let padding = " ".repeat(package_width.saturating_sub(visual_len));
            writeln!(f, "{name:name_width$}{versions}{padding}  {delta}")?;
        }

        writeln!(f)?;

        Ok(())
    }

    fn display_by_category(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let title_style = nu_ansi_term::Style::new()
            .underline()
            .bold()
            .fg(Color::LightGray);

        let name_width = self.longest_name + 2;

        if !self.changed.is_empty() {
            writeln!(f, "{}", &title_style.paint("Changed"))?;
            for (name, package) in &self.changed {
                writeln!(f, "[{}] {name:name_width$}{package}", Yellow.paint("C"))?;
            }
            writeln!(f)?;
        }

        if !self.added.is_empty() {
            writeln!(f, "{}", &title_style.paint("Added"))?;
            for (name, package) in &self.added {
                write!(f, "[{}] {name:name_width$}{package}", Green.paint("A"))?;
                writeln!(f)?;
            }
            writeln!(f)?;
        }

        if !self.removed.is_empty() {
            writeln!(f, "{}", &title_style.paint("Removed"))?;
            for (name, package) in &self.removed {
                writeln!(f, "[{}] {name:name_width$}{package}", Red.paint("R"))?;
            }
            writeln!(f)?;
        }

        Ok(())
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn from_diff_root(&mut self, diff_root: DiffRoot) {
        for (name, diff_package) in diff_root.packages {
            let package = Package::from(diff_package);

            self.size_delta.0 += package.size_delta.0;
            self.longest_name = self.longest_name.max(name.len());

            if self.by_size {
                self.all.push(PackageExtra {
                    name,
                    base_package: package,
                });

                continue;
            }

            match package.diff_type {
                DiffType::Added => {
                    self.added.insert(name, package);
                }
                DiffType::Removed => {
                    self.removed.insert(name, package);
                }
                DiffType::Changed => {
                    self.changed.insert(name, package);
                }
                DiffType::Unknown => {
                    // This should never happen, but just in case
                    eprintln!("Unknown diff type for package: {name}");
                }
            }
        }
    }
}
