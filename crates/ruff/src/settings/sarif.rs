use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Level {
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "note")]
    Note,
    #[serde(rename = "none")]
    None,
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub text: String,
    pub markdown: String,
}

#[derive(Serialize, Deserialize)]
pub struct MultiFormatMessageString {
    pub text: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub markdown: String,
}

#[derive(Serialize, Deserialize)]
pub struct ArtifactLocation {
    pub uri: String,
}

#[derive(Serialize, Deserialize)]
pub struct Region {
    #[serde(rename = "startLine")]
    pub start_line: usize,

    #[serde(rename = "startColumn")]
    pub start_column: usize,

    #[serde(rename = "endLine")]
    pub end_line: usize,

    #[serde(rename = "endColumn")]
    pub end_column: usize,
}

#[derive(Serialize, Deserialize)]
pub struct PhysicalLocation {
    #[serde(rename = "artifactLocation")]
    pub artifact_location: ArtifactLocation,
    pub region: Region,
}

#[derive(Serialize, Deserialize)]
pub struct Location {
    #[serde(rename = "physicalLocation")]
    pub physical_location: PhysicalLocation,
}

#[derive(Serialize, Deserialize)]
pub struct Result {
    #[serde(rename = "ruleId")]
    pub rule_id: String,

    pub message: Message,
    pub level: Level,
    pub locations: Vec<Location>,
}

#[derive(Serialize, Deserialize)]
pub struct SarifRule {
    pub id: String,
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SarifToolDriver {
    pub name: String,
    pub version: String,
    pub rules: Vec<SarifRule>,

    #[serde(rename = "shortDescription")]
    pub short_description: MultiFormatMessageString,
}

#[derive(Serialize, Deserialize)]
pub struct Tool {
    pub driver: SarifToolDriver,
}

#[derive(Serialize, Deserialize)]
pub struct Run {
    pub tool: Tool,
    pub results: Vec<Result>,
}

#[derive(Serialize, Deserialize)]
pub struct Report {
    pub version: String,

    #[serde(rename = "$schema")]
    pub schema_uri: Option<String>,

    pub runs: Vec<Run>,
}
