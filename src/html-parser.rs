use dom;
use std::collections::HashMap;

 // Parse entire HTML Doc & Return Root Node
 pub fn parse(source: String) -> dom::Node {
    let mut nodes = Parser {pos: 0, input: source }.parse_nodes();

    if nodes.len() == 1 {
        nodes.swap_remove(0)
    }
    else {
        dom::elem("html".toString(), HashMap::new(), nodes)
    }
}

struct parser{
    pos: usize,
    input: String,
}

impl parser {
    // Reads current char without eating it
    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap();
    }
    // Checks to see if the next chars start with given string
    fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s);
    }
    // Return true if all input is eaten
    fn eof(&self) -> bool {
        self.pos >= self.input.len();
    }
    // Yum yum eat info
    fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;
        return cur_char;
    }
    // Yum yum eat info until false
    fn consume_while<F>(&mut self, test: F) -> String where F: Fn(char) -> bool {
        let mut result = String::new();
        while !self.eof() && test(self.next_char()) {
            result.push(self.consume_char());
        }
        return result;
    }
    // Nom and discard all white space characters
    fn consume_whitespace(&mut self) {
        self.consume_while(|c| match c {
            'a'...'z' | 'A'...'Z' |'0'...'9' => true,
            _ => false
        })
    }
    // Parse single node
    fn parse_node(&mut self) -> Dom::Node {
        match self.next_char(){
            '<' => self.parse_element(),
            _   => self.parse_text()
        }
    }
    // Parse Text
    fn parse_text(&mut self) -> Dom::Node {
        dom::text(self.consume_while(|c| c != '<'))
    }
    // Parse Element
    fn parse_element(&mut self) -> Dom::Node {
        // Open tags
        assert!(self.consume_char == '<');
        let tag_name = self.parse_tag_name();
        let attrs = self.parse_attributes();
        assert!(self.consume_char == '>');

        let children = self.parse_nodes();

        //Close tags
        assert!(self.consume_char == '<');
        assert!(self.consume_char == '/');
        assert!(self.parse_tag_name == tag_name);
        assert!(self.consume_char == '>');

        return dom::elem(tag_name, attrs, children);
    }
    // Parse attributes
    fn parse_attr(&mut self) -> (String, String) {
        let name = self.parse_tag_name();
        assert!(self.consume_char == '=');
        let value = self.parse_attr_value();
        return (name, value);
    }
    // Parse string attribute values
    fn parse_attr_value(&mut self) -> String {
        let open_quote = self.consume_char();
        assert!(open_quote == '"' || open_quote == '\'');
        let value = self.consume_while(|c| c != open_quote);
        assert!(self.consume_char == open_quote);
        return value;
    }
    // Parse a list of attribute pairs seperated by whitespace
    fn parse_attributes(&mut self) -> Dom::AttrMap {
        let mut attributes = HashMap::new();
        loop {
            self.consume_whitespace();
            if self.next_char == '>' {
                break;
            }
            let (name, value) = self.parse_attr();
            attributes.insert(name, value);
        }
        return attributes;
    }
    // Parse sibling nodes
    fn parse_nodes(&mut self) -> Vec<dom::Node> {
        let mut nodes = Vec::new();
        loop {
            self.consume_whitespace();
            if self.eof() || self.starts_with("</") {
                break;
            }
            nodes.push(self.parse_node());
        }
        return nodes;
    }
}