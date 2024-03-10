use serde::{Deserialize, Serialize};

use super::mxcell::MxCell;

#[derive(Debug, Deserialize, Serialize, PartialEq, Default, Clone)]
pub struct Root {
    #[serde(rename = "mxCell")]
    pub elements: Vec<MxCell>,
}

fn default_dx() -> i32 {
    659
}

fn default_dy() -> i32 {
    250
}

fn default_grid_size() -> i32 {
    10
}

fn default_to_1() -> i32 {
    1
}

fn default_to_0() -> i32 {
    0
}

fn default_i32(value: i32) -> i32 {
    value
}

fn default_page_width() -> i32 {
    850
}

fn default_page_height() -> i32 {
    1100
}


#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(default)]
pub struct MxGraphModel {
    #[serde(default = "default_dx")]
    pub dx: i32,
    #[serde(default = "default_dy")]
    pub dy: i32,
    #[serde(default = "default_to_1")]
    pub grid: i32,
    #[serde(default = "default_grid_size")]
    #[serde(rename = "gridSize")]
    pub grid_size: i32,
    #[serde(default = "default_to_1")]
    pub guides: i32,
    #[serde(default = "default_to_1")]
    pub tooltips: i32,
    #[serde(default = "default_to_1")]
    pub connect: i32,
    #[serde(default = "default_to_1")]
    pub arrows: i32,
    #[serde(default = "default_to_1")]
    pub fold: i32,
    #[serde(default = "default_to_1")]
    pub page: i32,
    #[serde(default = "default_to_1")]
    #[serde(rename = "pageScale")]
    pub page_scale: i32,
    #[serde(default = "default_page_width")]
    #[serde(rename = "pageWidth")]
    pub page_width: i32,
    #[serde(default = "default_page_height")]
    #[serde(rename = "pageHeight")]
    pub page_height: i32,
    #[serde(default = "default_to_0")]
    pub math: i32,
    #[serde(default = "default_to_0")]
    pub shadow: i32,
    #[serde(rename = "root")]
    pub root: Root,
}

impl Default for MxGraphModel {
    fn default() -> Self {
        Self { 
            dx: default_dx(),
            dy: default_dy(),
            arrows: default_to_1(),
            grid: default_to_1(),
            connect: default_to_1(),
            fold: default_to_1(),
            grid_size: default_grid_size(),
            guides: default_to_1(),
            math: default_to_0(),
            shadow: default_to_0(),
            page: default_to_1(),
            page_height: default_page_height(),
            page_scale: default_to_1(),
            page_width: default_page_width(),
            tooltips: default_to_1(),
            root: Root::default()
        }
    }
}
