use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Cpu {
    #[serde(rename = "@mode")]
    pub mode: String,
    #[serde(rename = "@match")]
    pub cpu_match: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub model: Model,
    pub vendor: String,
    pub maxphysaddr: Maxphysaddr,
    pub feature: Vec<Feature>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Model {
    #[serde(rename = "@fallback")]
    pub fallback: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Maxphysaddr {
    #[serde(rename = "@mode")]
    pub mode: String,
    #[serde(rename = "@limit")]
    pub limit: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Feature {
    #[serde(rename = "@policy")]
    pub policy: String,
    #[serde(rename = "@name")]
    pub name: String,
}
