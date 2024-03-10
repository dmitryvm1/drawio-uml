use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Default, Clone)]
pub struct UserObject {
    pub label: Option<String>,
    pub link: Option<String>,
}

impl UserObject {
    pub fn get_link(&self) -> Option<(String, String)> {
        if let Some(label) = self.label.clone() {
            if let Some(url) = self.link.clone() {
                return Some((url, cleanup_label(label)));
            }
        }
        None
    }
}

pub fn cleanup_label(text: String) -> String {
    let raw_label = text
        .replace("&nbsp;", " ")
        .replace("<br>", " ")
        .replace("<b>", " ")
        .replace("</b>", " ")
        .replace("<u>", " ")
        .replace("</u>", " ")
        .replace("<i>", " ")
        .replace("</i>", " ")
        .replace("<strike>", " ")
        .replace("</strike>", " ")
        .replace("<span>", " ")
        .replace("</span>", " ");
    let trimmed_label = raw_label.as_str().trim();
    let remove_multiple_whitespaces = Regex::new(r"\s+").unwrap();
    remove_multiple_whitespaces
        .replace_all(trimmed_label, " ")
        .into_owned()
}

