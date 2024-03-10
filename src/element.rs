use serde::{Deserialize, Serialize, Deserializer};

use super::{mxcell::MxCell, userobject::UserObject};

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub enum Element {
    #[serde(rename = "mxCell")]
    MxCell(MxCell),
    UserObject(UserObject),
    #[serde(other, deserialize_with = "deserialize_ignore_any")]
    Other,
}

fn deserialize_ignore_any<'de, D: Deserializer<'de>>(deserializer: D) -> Result<(), D::Error> {
    serde::de::IgnoredAny::deserialize(deserializer)?;
    Ok(())
}