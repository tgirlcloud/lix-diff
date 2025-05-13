use color_eyre::Result;
use serde::de::Deserializer;
use serde::Deserialize;
use std::{borrow::Cow, collections::BTreeMap, process::Command};

#[derive(Deserialize, Debug)]
pub struct DiffRoot {
    pub packages: BTreeMap<String, DiffPackage>,

    #[expect(dead_code)]
    pub schema: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DiffPackage {
    pub size_delta: i64,

    #[serde(deserialize_with = "version_deserializer")]
    pub versions_before: Vec<String>,

    #[serde(deserialize_with = "version_deserializer")]
    pub versions_after: Vec<String>,
}

fn version_deserializer<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let vec = Vec::<Cow<'de, str>>::deserialize(deserializer)?;
    Ok(vec
        .into_iter()
        .map(|s| {
            if s.is_empty() {
                "<none>".to_string()
            } else {
                s.into_owned()
            }
        })
        .collect())
}

impl DiffRoot {
    pub fn new(before: &str, after: &str) -> Result<DiffRoot> {
        let raw_diff = Command::new("nix")
            .args(["store", "diff-closures", "--json", before, after])
            .output()?;

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
        let diff_out = String::from_utf8(stdout)?;

        let diff_root = serde_json::from_str::<DiffRoot>(&diff_out).map_err(|e| {
            eprintln!("Failed to parse JSON: {e}");
            std::process::exit(1);
        })?;

        Ok(diff_root)
    }
}
