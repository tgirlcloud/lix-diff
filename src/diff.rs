use humansize::{format_size, DECIMAL};
use nu_ansi_term::Color::{self, Green, Red, Yellow};
use std::collections::BTreeMap;

use crate::package::{DiffType, Package};
use crate::parser::DiffRoot;

#[derive(Debug)]
pub struct PackageListDiff {
    pub added: BTreeMap<String, Package>,
    pub removed: BTreeMap<String, Package>,
    pub changed: BTreeMap<String, Package>,
    size_delta: i64,
    longest_name: usize,
}

pub fn partition_diff(diff: &DiffRoot) -> PackageListDiff {
    let mut added = BTreeMap::new();
    let mut removed = BTreeMap::new();
    let mut changed = BTreeMap::new();
    let mut size_delta: i64 = 0;
    let mut longest_name = 0;

    for (name, package) in &diff.packages {
        match package.diff_type {
            DiffType::Added => {
                added.insert(name.clone(), package.clone());
            }
            DiffType::Removed => {
                removed.insert(name.clone(), package.clone());
            }
            DiffType::Changed => {
                changed.insert(name.clone(), package.clone());
            }
            _ => {}
        }

        size_delta += package.size_delta;
        longest_name = longest_name.max(name.len());
    }

    PackageListDiff {
        added,
        removed,
        changed,
        size_delta,
        longest_name,
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
