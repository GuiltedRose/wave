use std::collections::HashMap;

type AttrMap = HashMap<String, String>;

struct Node {
    children: Vec<Node>,
    node_type: NodeType,
}

enum NodeType {
    text(String),
    Element(ElementData),
}

struct ElementData {
    tag_name: String,
    attributes: AttrMap,
}

fn text(data: String) -> Node {
    Node {
        children, children,
        node_type: NodeType::Element(ElementData {
            tag_name: name,
            attributes: attrs,
        })
    }
}