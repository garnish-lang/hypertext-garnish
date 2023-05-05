enum DeclarationValue {
    Basic(String),
    Function(String, Vec<String>), // (function name, function arguments
}

impl ToString for DeclarationValue {
    fn to_string(&self) -> String {
        match self {
            DeclarationValue::Basic(s) => s.to_string(),
            DeclarationValue::Function(name, args) => format!("{}({})", name, args.join(", ")),
        }
    }
}

struct Declaration {
    property: String,
    value: DeclarationValue,
}

impl Declaration {
    pub fn new(property: String, value: DeclarationValue) -> Self {
        Self { property, value }
    }
}

impl ToString for Declaration {
    fn to_string(&self) -> String {
        format!("{}: {};", self.property, self.value.to_string())
    }
}

enum Combinator {
    Descendant,
    Child,
    AdjacentSibling,
    GeneralSibling,
}

enum Selector {
    Tag(String),                                          // tag name
    Class(String),                                        // class name
    Id(String),                                           // id name
    Combinator(Box<Selector>, Combinator, Box<Selector>), // (base selector, combination)
    PseudoClass(Box<Selector>, String),                   // (base selector, pseudo class)
    PseudoElement(Box<Selector>, String),                 // (base selector, pseudo element)
    Attribute(String),                                    // attribute name
    AttributeValue(String, String),                       // (attribute name, attribute value)
    AttributeContains(String, String),                    // (attribute name, search string)
    Chain(Vec<Selector>), // no space merge (e.g. p.my-class[someAttribute])
    Group(Vec<Selector>), // comma separated list (e.g. body, h1, p)
}

struct Rule {
    selector: Selector,
    declarations: Vec<Declaration>,
    sub_rules: Vec<Rule>,
}

struct RuleSet {
    media_query: Option<String>,
    rules: Vec<Rule>,
    sub_sets: Vec<RuleSet>,
}

#[cfg(test)]
mod to_string {
    use crate::css::{Declaration, DeclarationValue};

    #[test]
    fn declaration() {
        let d = Declaration::new(
            "color".to_string(),
            DeclarationValue::Basic("blue".to_string()),
        );
        assert_eq!(d.to_string(), "color: blue;")
    }

    #[test]
    fn declaration_with_function() {
        let d = Declaration::new(
            "color".to_string(),
            DeclarationValue::Function(
                "rgb".to_string(),
                vec!["200".into(), "200".into(), "200".into()],
            ),
        );
        assert_eq!(d.to_string(), "color: rgb(200, 200, 200);")
    }
}
