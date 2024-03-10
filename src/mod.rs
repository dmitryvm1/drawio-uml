pub mod mxcell;
pub mod mxgeometry;
pub mod userobject;
pub mod style;
pub mod diagram;
pub mod mxfile;
pub mod mxgraphmodel;
pub mod element;


mod test{
    use crate::drawio::{element::Element, diagram::Diagram, mxcell::MxCell, mxgeometry::MxGeometry, mxgraphmodel::MxGraphModel, style::StyleBuilder};
    use super::mxfile::{Mxfile, read_file};

    #[test]
    fn test_mxfile() {
        let mxfile_data = read_file(std::path::Path::new("./uml.drawio")).unwrap();
        println!("{:?}", mxfile_data);
    }
    
    #[test]
    fn test_write_mxfile() {
        let mxfile_data = read_file(std::path::Path::new("./uml.drawio")).unwrap();
        let contents = fast_xml::se::to_string(&mxfile_data).unwrap();
        println!("{}", contents);
        std::fs::write("test.drawio", contents).unwrap();
    
    }
    
    #[test]
    fn test_write_empty_mxfile() {
        let mut mxfile_data = Mxfile::default();
        let field_count = 2;
        let field_height = 50;
        // Create a page (which is also called a diagram)
        // 6SMVXu_SVDnK60xZtHuv
        let mut diagram = Diagram { id: "6SMVXu_SVDnK60x".to_string(), name: "Page-1".to_string(), mx_graph_model: MxGraphModel::default() };
        // Root mxcell. Just has to be there.
        let mut cell = MxCell {
            geometry: None,
            id: "0".to_string(),
            parent: None,
            value: None,
            style: None,
            vertex: None
        };
        diagram.mx_graph_model.root.elements.push(cell.clone());
        // We have to make this more readable.
        // There must be a layouter which will track the position of 
        // each class shape.
        // Each class shape will have a header and fileds.
        // Need a trait Positionable so layouter can operate on all kinds of shapes.
        // a structure ClassShape shoulb be transformed into set of mxcells in case of drawio 
        // presentation layer. 
        cell.parent = Some(cell.id.clone());
        cell.id = "1".to_string();
        diagram.mx_graph_model.root.elements.push(cell.clone());
        cell.parent = Some(cell.id.clone());
        cell.id = "afwe35sfwe2r-2342".to_string();
        cell.value = Some("Struct Name".to_string());
        cell.vertex = Some(1);
        cell.style = Some(StyleBuilder::class_shape());
        cell.geometry = Some(MxGeometry {
            height: field_count + 1 * field_height,
            ..Default::default()
        });
        diagram.mx_graph_model.root.elements.push(cell.clone());

        // Fields
        cell.parent = Some(cell.id.clone());
        cell.id = "afwe35sfwe2r-1".to_string();
        cell.value = Some("x: i32".to_string());
        cell.style = Some(StyleBuilder::class_field());
        cell.geometry = Some(MxGeometry {
            x: 0,
            y: 50,
            ..Default::default()
        });
        diagram.mx_graph_model.root.elements.push(cell.clone());

        cell.id = "afwe35sfwe2r-2".to_string();
        cell.value = Some("y: i32".to_string());
        // cell.style = Some(StyleBuilder::class_field());
        cell.geometry = Some(MxGeometry {
            x: 0,
            y: 100,
            ..Default::default()
        });
        diagram.mx_graph_model.root.elements.push(cell.clone());
        mxfile_data.diagrams.push(diagram);
        let contents = fast_xml::se::to_string(&mxfile_data).unwrap();
        println!("{}", contents);
        std::fs::write("empty_test.drawio", contents).unwrap();
    
    }
}
