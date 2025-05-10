use color_eyre::Result;
use serde::Deserialize;
use std::{collections::BTreeMap, process::Command};

use crate::package::{DiffType, Package};

#[derive(Deserialize, Debug)]
pub struct DiffRoot {
    pub packages: BTreeMap<String, Package>,

    #[expect(dead_code)]
    pub schema: String,
}

fn run_diff(before: &str, after: &str) -> String {
    let raw_diff = Command::new("nix")
        .args(["store", "diff-closures", "--json", before, after])
        .output()
        .expect("Failed to execute command");

    if !raw_diff.status.success() {
        eprintln!("Error: {}", String::from_utf8_lossy(&raw_diff.stderr));
        std::process::exit(1);
    }

    let diff_output = String::from_utf8_lossy(&raw_diff.stdout);
    if diff_output.is_empty() {
        eprintln!("No differences found.");
        std::process::exit(0);
    }

    diff_output.into_owned()
}

fn parse_diff(input: &str) -> Result<DiffRoot> {
    serde_json::from_str(input)
        .map(|mut diff_root: DiffRoot| {
            for package in diff_root.packages.values_mut() {
                package.diff_type =
                    DiffType::from_versions(&package.versions_before, &package.versions_after);
            }
            diff_root
        })
        .map_err(|e| {
            eprintln!("Failed to parse JSON: {e}");
            std::process::exit(1);
        })
}

pub fn diff(before: &str, after: &str) -> Result<DiffRoot> {
    let diff_output = run_diff(before, after);
    let diff_root: DiffRoot = parse_diff(&diff_output)?;

    Ok(diff_root)
}
