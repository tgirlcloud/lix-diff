use humansize::{format_size, DECIMAL};
use nu_ansi_term::Color::{self, Green, Red, Yellow};
use std::collections::BTreeMap;

use super::{
    package::{DiffType, Package},
    parser::DiffRoot,
};

#[derive(Debug)]
pub struct PackageListDiff {
    pub added: BTreeMap<String, Package>,
    pub removed: BTreeMap<String, Package>,
    pub changed: BTreeMap<String, Package>,
    size_delta: i64,
    longest_name: usize,
}

impl From<DiffRoot> for PackageListDiff {
    fn from(diff: DiffRoot) -> Self {
        let mut out = PackageListDiff {
            added: BTreeMap::new(),
            removed: BTreeMap::new(),
            changed: BTreeMap::new(),
            size_delta: 0,
            longest_name: 0,
        };

        for (name, diff_package) in diff.packages {
            let package = Package::from(diff_package);

            out.size_delta += package.size_delta;
            out.longest_name = out.longest_name.max(name.len());

            match package.diff_type {
                DiffType::Added => {
                    out.added.insert(name, package);
                }
                DiffType::Removed => {
                    out.removed.insert(name, package);
                }
                DiffType::Changed => {
                    out.changed.insert(name, package);
                }
                DiffType::Unknown => {
                    // This should never happen, but just in case
                    eprintln!("Unknown diff type for package: {name}");
                }
            }
        }

        out
    }
}

impl std::fmt::Display for PackageListDiff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.added.is_empty() && self.removed.is_empty() && self.changed.is_empty() {
            return write!(f, "No differences found.");
        }

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

        {
            let delta = self.size_delta;
            let sign = if delta > 0 { "+" } else { "-" };
            let size: u64 = delta.abs().try_into().unwrap_or(0);
            writeln!(f, "size diff: {sign}{}", format_size(size, DECIMAL))?;
        }

        Ok(())
    }
}
