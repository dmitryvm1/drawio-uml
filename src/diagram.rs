use serde::{Deserialize, Serialize};
use crate::mxcell::MxCell;

use super::mxgraphmodel::MxGraphModel;

#[derive(Debug, Deserialize, Serialize, PartialEq, Default, Clone)]
pub struct Diagram {
    pub id: String,
    pub name: String,
    #[serde(rename = "mxGraphModel", default)]
    pub mx_graph_model: MxGraphModel,
}

impl Diagram {
    // pub fn get_links(&self) -> Vec<(String, String)> {
    //     self.mx_graph_model
    //         .root
    //         .elements
    //         .iter()
    //         .filter_map(|element| match element {
    //             Element::MxCell(cell) => cell.get_link(),
    //             Element::UserObject(user_object) => user_object.get_link(),
    //             Element::Other => None,
    //         })
    //         .collect()
    // }

    pub fn add_cell(&mut self, cell: MxCell) {
        self.mx_graph_model.root.elements.push(cell);
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Default, Clone)]
pub struct CompressDiagram {
    pub id: String,
    pub name: String,
    #[serde(rename = "$value")]
    pub raw_diagram: String,
}