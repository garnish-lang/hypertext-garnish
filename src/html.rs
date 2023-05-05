struct Attribute {
    name: String,
    value: Option<String>,
}

impl Attribute {
    pub fn new(name: String, value: String) -> Self {
        Self { name, value: Some(value) }
    }

    pub fn toggle(name: String) -> Self {
        Self { name, value: None }
    }
}

impl ToString for Attribute {
    fn to_string(&self) -> String {
        match &self.value {
            Some(value) => {
                format!("{}=\"{}\"", self.name, value)
            }
            None => self.name.to_string()
        }
    }
}

enum Node {
    Text(String),
    Element {
        tag: String,
        attributes: Vec<Attribute>,
        children: Vec<Node>,
    },
}

impl Node {
    pub fn element(tag: String, attributes: Vec<Attribute>, children: Vec<Node>) -> Self {
        Self::Element {
            tag,
            attributes,
            children,
        }
    }

    pub fn text(text: String) -> Self {
        Self::Text(text)
    }
}

impl ToString for Node {
    fn to_string(&self) -> String {
        match self {
            Node::Text(s) => s.clone(),
            Node::Element {
                tag,
                attributes,
                children,
            } => {
                let child_text = children
                    .iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<String>>()
                    .join("");

                let open_tag = match attributes.is_empty() {
                    true => format!("<{}>", tag),
                    false => {
                        let attribute_text = attributes
                            .iter()
                            .map(Attribute::to_string)
                            .collect::<Vec<String>>()
                            .join(" ");
                        format!("<{} {}>", tag, attribute_text)
                    }
                };
                format!("{}{}</{}>", open_tag, child_text, tag)
            }
        }
    }
}

#[cfg(test)]
mod to_string {
    use crate::html::{Attribute, Node};

    #[test]
    fn single_element() {
        let element = Node::element("body".to_string(), vec![], vec![]);

        assert_eq!(element.to_string(), "<body></body>");
    }

    #[test]
    fn single_element_with_text() {
        let element = Node::element(
            "body".to_string(),
            vec![],
            vec![Node::text("Some text".to_string())],
        );

        assert_eq!(element.to_string(), "<body>Some text</body>");
    }

    #[test]
    fn attribute() {
        let attr = Attribute::new("class".to_string(), "my-class".to_string());
        assert_eq!(attr.to_string(), "class=\"my-class\"")
    }

    #[test]
    fn attribute_no_value() {
        let attr = Attribute::toggle("class".to_string());
        assert_eq!(attr.to_string(), "class")
    }

    #[test]
    fn single_element_with_attributes() {
        let element = Node::element(
            "body".to_string(),
            vec![
                Attribute::new("class".to_string(), "my-class".to_string()),
                Attribute::new("width".to_string(), "100".to_string()),
            ],
            vec![],
        );

        assert_eq!(
            element.to_string(),
            "<body class=\"my-class\" width=\"100\"></body>"
        );
    }

    #[test]
    fn child_elements() {
        let element = Node::element(
            "body".to_string(),
            vec![],
            vec![Node::element(
                "h1".to_string(),
                vec![],
                vec![Node::text("Heading".to_string())],
            )],
        );

        assert_eq!(element.to_string(), "<body><h1>Heading</h1></body>");
    }

    #[test]
    fn child_elements_then_text() {
        let element = Node::element(
            "body".to_string(),
            vec![],
            vec![
                Node::element(
                    "h1".to_string(),
                    vec![],
                    vec![Node::text("Heading".to_string())],
                ),
                Node::text("Some text".to_string()),
            ],
        );

        assert_eq!(
            element.to_string(),
            "<body><h1>Heading</h1>Some text</body>"
        );
    }
}
