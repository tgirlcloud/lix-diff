use nu_ansi_term::Color::{Green, Red};
use serde::de::Deserializer;
use serde::Deserialize;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiffType {
    Added,
    Removed,
    Changed,

    #[default]
    Unknown,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Package {
    pub size_delta: i64,

    #[serde(deserialize_with = "version_deserializer")]
    pub versions_before: Vec<String>,

    #[serde(deserialize_with = "version_deserializer")]
    pub versions_after: Vec<String>,

    /// This is not a part of the JSON schema, but is used to determine the type of diff
    #[serde(skip)]
    pub diff_type: DiffType,
}

fn version_deserializer<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let vec = Vec::<String>::deserialize(deserializer)?;
    Ok(vec
        .into_iter()
        .map(|s| {
            if s.is_empty() {
                "<none>".to_string()
            } else {
                s
            }
        })
        .collect())
}

impl DiffType {
    pub fn from_versions(before: &[String], after: &[String]) -> DiffType {
        match (before.is_empty(), after.is_empty()) {
            (true, false) => DiffType::Added,
            (false, true) => DiffType::Removed,
            (false, false) => DiffType::Changed,
            (true, true) => DiffType::Unknown, // should be unreachable but im not sure
        }
    }
}

impl std::fmt::Display for Package {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // added package
        match self.diff_type {
            DiffType::Added => {
                write!(f, "{}", Green.paint(self.versions_after.join(", ")))?;
            }
            DiffType::Removed => {
                write!(f, "{}", Red.paint(self.versions_before.join(", ")))?;
            }
            DiffType::Changed => {
                write!(
                    f,
                    "{} -> {}",
                    Red.paint(self.versions_before.join(", ")),
                    Green.paint(self.versions_after.join(", "))
                )?;
            }
            DiffType::Unknown => unreachable!(),
        }

        Ok(())
    }
}
