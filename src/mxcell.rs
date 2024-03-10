use regex::Regex;
use serde::{Deserialize, Serialize};

use super::{userobject::cleanup_label, mxgeometry::MxGeometry};

#[derive(Debug, Deserialize, Serialize, PartialEq, Default, Clone)]
#[serde(rename = "mxCell")]
pub struct MxCell {
    #[serde(default = "default_to_0")]
    pub id: String,
    pub parent: Option<String>,
    pub value: Option<String>,
    pub style: Option<String>,
    pub vertex: Option<i32>,
    #[serde(rename = "mxGeometry")]
    pub geometry: Option<MxGeometry>
}

impl MxCell {
    pub fn get_link(&self) -> Option<(String, String)> {
        if let Some(value) = self.value.clone() {
            if value.contains("href=") {
                return MxCell::extract_link(value);
            }
        }
        None
    }

    fn extract_link(value: String) -> Option<(String, String)> {
        if let Ok(re) = Regex::new(".*href=\"(.*)\".*>(.*)<.*") {
            if let Some(caps) = re.captures(value.as_str()) {
                let link_url = caps.get(1).map(|link| link.as_str().to_string());
                let link_label = caps.get(2).map(|link| link.as_str().to_string());

                if let (Some(url), Some(label)) = (link_url, link_label) {
                    return Some((url, cleanup_label(label)));
                }
            }
        }
        None
    }
}

fn default_to_0() -> String {
    "0".to_string()
}