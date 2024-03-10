## Drawio UML

A tool to programmatically create blocks of class diagrams.

### Example

```
    let mut diagram = new_diagram("Page 1");
    let mut mxfile = Mxfile::default();
    let seq_id = SeqId::new();
    let mut cls = UMLClass::new("Classname", "1", seq_id.clone(), 140);
    cls.add_field("- field 1");
    cls.add_field("- field 2");
    cls.add_field("- field 3");
    cls.set_position(600, 200);
    diagram.mx_graph_model.root.elements.append(&mut cls.into_cells());

    let mut cls = UMLClass::new("Classname", "1", seq_id.clone(), 140);
    cls.add_field("- field 11");
    cls.add_field("- field 22");
    cls.add_field("- field 33");
    cls.add_field("- field 22");
    cls.add_field("- field 33");
    cls.set_position(100, 200);
    diagram.mx_graph_model.root.elements.append(&mut cls.into_cells());
    mxfile.diagrams.push(diagram);
    std::fs::write("test_out.drawio", fast_xml::se::to_string(&mxfile).unwrap()).unwrap();

```