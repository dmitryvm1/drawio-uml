
use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename = "mxGeometry")]
pub struct MxGeometry {
    pub x: Option<i32>,
    pub y: Option<i32>,
    pub width: i32,
    pub height: i32,
    pub r#as: String
}

impl Default for MxGeometry {
    fn default() -> MxGeometry {
        MxGeometry { x: Some(0), y: Some(0), width: 110, height: 50, r#as: "geometry".to_string() }
    }
}