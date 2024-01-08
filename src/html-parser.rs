struct Element {
    tag_name: String,
    attributes: HashMap<String, String>,
    children: Vec<Element>
}

fn parse_html(html: &str) -> Element {
    let root_element = Element {
        tag_name: "html".to_string(),
        attributes: HashMap::new(),
        children: vec! [],

    };

    root_element
}