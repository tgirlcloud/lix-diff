use color_eyre::Result;
use serde::Deserialize;
use std::{collections::BTreeMap, process::Command};

use crate::package::{DiffType, Package};

#[derive(Debug)]
pub struct DiffRoot {
    pub packages: BTreeMap<String, Package>,

    #[expect(dead_code)]
    pub schema: String,
}

impl<'de> Deserialize<'de> for DiffRoot {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Raw {
            packages: BTreeMap<String, Package>,
            schema: String,
        }

        let Raw {
            mut packages,
            schema,
        } = Raw::deserialize(deserializer)?;

        for pkg in packages.values_mut() {
            pkg.diff_type = DiffType::from_versions(&pkg.versions_before, &pkg.versions_after);
        }

        Ok(DiffRoot { packages, schema })
    }
}

fn run_diff(before: &str, after: &str) -> String {
    let raw_diff = Command::new("nix")
        .args(["store", "diff-closures", "--json", before, after])
        .output()
        .expect("Failed to execute nix command");

    if !raw_diff.status.success() {
        eprintln!("{}", String::from_utf8_lossy(&raw_diff.stderr));
        std::process::exit(1);
    }

    let stdout = raw_diff.stdout;
    if stdout.is_empty() {
        eprintln!("No differences found.");
        std::process::exit(0);
    }

    // Assume nix output is valid UTF-8
    String::from_utf8(stdout).expect("Output was not valid UTF-8")
}

fn parse_diff(input: &str) -> Result<DiffRoot> {
    serde_json::from_str::<DiffRoot>(input).map_err(|e| {
        eprintln!("Failed to parse JSON: {e}");
        std::process::exit(1);
    })
}

pub fn diff(before: &str, after: &str) -> Result<DiffRoot> {
    let diff_output = run_diff(before, after);
    let diff_root: DiffRoot = parse_diff(&diff_output)?;

    Ok(diff_root)
}
