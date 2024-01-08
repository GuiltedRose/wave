struct Element {
    tag_name: String,
    attributes: HashMap<String, String>,
    children: Vec<Element>
}

fn parse_html(html: &str) -> Element {
    let mut root_element: Option<Element> = None;
    let mut current_element: Option<Element> = None;
    for token in html.split_whitespace(){
        match token{
            "<html>" => {
                let element = Element {
                    tag_name: "html".to_string(),
                    attributes: HashMap::new(),
                    children: Vec::new(),
                };
                root_element = Some(element);
                current_element = root_element.clone();
            }
            "</html>" => {
                current_element = None;
            }
        }
    }

    root_element.unwrap()
}