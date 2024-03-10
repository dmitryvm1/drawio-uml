use std::{cell::Cell, rc::Rc};

use crate::{diagram::Diagram, mxcell::MxCell, mxgeometry::MxGeometry};

pub trait IdGen: Sized {
    fn next(&mut self) -> usize;
}

#[derive(Clone)]
pub struct SeqId {
    next_id: Rc<Cell<usize>>
}

impl SeqId {
    pub fn new() -> Self {
        SeqId {
            next_id: Rc::new(Cell::new(3))
        }
    }
}

impl IdGen for SeqId {
    fn next(&mut self) -> usize {
        let id: usize = self.next_id.get();
        self.next_id.set(id + 1);
        id
    }
}

/// Creates a diagram. Sets the root cell id = "1"
pub fn new_diagram(page_name: &str) -> Diagram {
    let mut diagram = Diagram::default();
    diagram.name = page_name.to_string();
    let mut cell_0 = MxCell::default();
    cell_0.id = "0".to_string();
    let mut cell_1 = MxCell::default();
    cell_1.parent = Some("0".to_string());
    cell_1.id = "1".to_string();
    diagram.mx_graph_model.root.elements.push(cell_0);
    diagram.mx_graph_model.root.elements.push(cell_1);
    diagram
}

pub struct UMLClass<IDGEN: IdGen> {
    cells: Vec<MxCell>,
    id_gen: IDGEN,
    next_y: i32,
    width: i32
}

impl<IDGEN: IdGen> UMLClass<IDGEN> {
    /// Default width: 140
    pub fn new(cls_name: &str, root_id: &str, mut id_gen: IDGEN, width: i32) -> Self {
        let mut cells = Vec::new();
        let mut cell = MxCell::default();
        cell.id = id_gen.next().to_string();
        cell.parent = Some(root_id.to_string());
        cell.value = Some(cls_name.to_string());
        cell.vertex = Some(1);
        cell.style = Some("swimlane;fontStyle=0;childLayout=stackLayout;horizontal=1;startSize=26;fillColor=none;horizontalStack=0;resizeParent=1;resizeParentMax=0;resizeLast=0;collapsible=1;marginBottom=0;".to_string());
        cell.geometry = Some(MxGeometry {
            x: Some(40),
            y: Some(80),
            width: width,
            height: 104,
            r#as: "geometry".to_string()
        });
        cells.push(cell);
        UMLClass {
            cells,
            id_gen,
            next_y: 26,
            width
        }
    }

    pub fn add_field(&mut self, name: &str) {
        let parent_id = self.cells[0].id.clone();
        let mut cell = MxCell::default();
        cell.id = self.id_gen.next().to_string();
        cell.parent = Some(parent_id);
        cell.value = Some(name.to_string());
        cell.vertex = Some(1);
        cell.style = Some("text;strokeColor=none;fillColor=none;align=left;verticalAlign=top;spacingLeft=4;spacingRight=4;overflow=hidden;rotatable=0;points=[[0,0.5],[1,0.5]];portConstraint=eastwest;".to_string());
        cell.geometry = Some(MxGeometry {
            x: None,
            y: Some(self.next_y),
            width: self.width,
            height: 26,
            r#as: "geometry".to_string()
        });
        self.next_y += 26;
        self.cells.get_mut(0).as_mut().unwrap().geometry.as_mut().unwrap().height = self.next_y;
        self.cells.push(cell);
    }

    pub fn cells(&self) -> &[MxCell] {
        self.cells.as_slice()
    }

    pub fn into_cells(self) -> Vec<MxCell> {
        self.cells
    }

    pub fn set_position(&mut self, x: i32, y: i32) {
        self.cells.get_mut(0).as_mut().unwrap().geometry.as_mut().unwrap().x = Some(x);
        self.cells.get_mut(0).as_mut().unwrap().geometry.as_mut().unwrap().y = Some(y);

    }
}


#[cfg(test)]
mod tests {
    use super::{SeqId, UMLClass};

    #[test]
    fn test_new_class() {
        let seq_id = SeqId::new();
        let mut cls = UMLClass::new("Classname", "1", seq_id.clone(), 140);
        cls.add_field("- field 1");
        cls.add_field("- field 2");
        cls.add_field("- field 3");
        assert_eq!(cls.cells.len(), 4);
    }
}