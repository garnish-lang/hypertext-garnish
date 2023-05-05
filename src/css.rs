enum DeclarationValue {
    Basic(String),
    Function(String, Vec<String>), // (function name, function arguments
}

impl ToString for DeclarationValue {
    fn to_string(&self) -> String {
        match self {
            DeclarationValue::Basic(s) => match s.contains(" ") {
                true => format!("\"{}\"", s),
                false => s.to_string(),
            },
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
        format!("{}:{};", self.property, self.value.to_string())
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

impl ToString for Selector {
    fn to_string(&self) -> String {
        match self {
            Selector::Tag(s) => s.to_string(),
            Selector::Id(id) => format!("#{}", id),
            Selector::Class(class) => format!(".{}", class),
            Selector::Combinator(base, op, relative) => {
                format!(
                    "{}{} {}",
                    base.to_string(),
                    match op {
                        Combinator::Descendant => "",
                        Combinator::Child => " >",
                        Combinator::AdjacentSibling => " +",
                        Combinator::GeneralSibling => " ~",
                    },
                    relative.to_string()
                )
            }
            Selector::PseudoClass(base, class) => format!("{}:{}", base.to_string(), class),
            Selector::PseudoElement(base, class) => format!("{}::{}", base.to_string(), class),
            Selector::Attribute(attr) => format!("[{}]", attr),
            Selector::AttributeValue(attr, value) => format!("[{}=\"{}\"]", attr, value),
            Selector::AttributeContains(attr, value) => format!("[{}~=\"{}\"]", attr, value),
            Selector::Chain(items) => items
                .iter()
                .map(Selector::to_string)
                .collect::<Vec<String>>()
                .join(""),
            Selector::Group(items) => items
                .iter()
                .map(Selector::to_string)
                .collect::<Vec<String>>()
                .join(", "),
        }
    }
}

struct Rule {
    selector: Selector,
    declarations: Vec<Declaration>,
    sub_rules: Vec<Rule>,
}

impl Rule {
    pub fn new(selector: Selector, declarations: Vec<Declaration>, sub_rules: Vec<Rule>) -> Self {
        Self {
            selector,
            declarations,
            sub_rules,
        }
    }
}

impl ToString for Rule {
    fn to_string(&self) -> String {
        format!(
            "{}{{{}}}",
            self.selector.to_string(),
            self.declarations
                .iter()
                .map(Declaration::to_string)
                .collect::<Vec<String>>()
                .join("")
        )
    }
}

struct RuleSet {
    media_query: Option<String>,
    rules: Vec<Rule>,
    sub_sets: Vec<RuleSet>,
}

#[cfg(test)]
mod to_string {
    use crate::css::{Combinator, Declaration, DeclarationValue, Rule, Selector};

    #[test]
    fn declaration() {
        let d = Declaration::new(
            "color".to_string(),
            DeclarationValue::Basic("blue".to_string()),
        );
        assert_eq!(d.to_string(), "color:blue;")
    }

    #[test]
    fn declaration_basic_quotes_strings_with_spaces() {
        let d = Declaration::new(
            "font-family".to_string(),
            DeclarationValue::Basic("Times New Roman".to_string()),
        );
        assert_eq!(d.to_string(), "font-family:\"Times New Roman\";")
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
        assert_eq!(d.to_string(), "color:rgb(200, 200, 200);")
    }

    #[test]
    fn tag_selector() {
        let s = Selector::Tag("body".to_string());

        assert_eq!(s.to_string(), "body");
    }

    #[test]
    fn class_selector() {
        let s = Selector::Class("my-class".to_string());

        assert_eq!(s.to_string(), ".my-class");
    }

    #[test]
    fn id_selector() {
        let s = Selector::Id("my_id".to_string());

        assert_eq!(s.to_string(), "#my_id");
    }

    #[test]
    fn combinator_descendant() {
        let s = Selector::Combinator(
            Box::new(Selector::Tag("body".to_string())),
            Combinator::Descendant,
            Box::new(Selector::Tag("h1".to_string())),
        );

        assert_eq!(s.to_string(), "body h1");
    }

    #[test]
    fn combinator_child() {
        let s = Selector::Combinator(
            Box::new(Selector::Tag("body".to_string())),
            Combinator::Child,
            Box::new(Selector::Tag("h1".to_string())),
        );

        assert_eq!(s.to_string(), "body > h1");
    }

    #[test]
    fn combinator_adjacent_sibling() {
        let s = Selector::Combinator(
            Box::new(Selector::Tag("body".to_string())),
            Combinator::AdjacentSibling,
            Box::new(Selector::Tag("h1".to_string())),
        );

        assert_eq!(s.to_string(), "body + h1");
    }

    #[test]
    fn combinator_general_sibling() {
        let s = Selector::Combinator(
            Box::new(Selector::Tag("body".to_string())),
            Combinator::GeneralSibling,
            Box::new(Selector::Tag("h1".to_string())),
        );

        assert_eq!(s.to_string(), "body ~ h1");
    }

    #[test]
    fn combinator_multiple() {
        let s = Selector::Combinator(
            Box::new(Selector::Combinator(
                Box::new(Selector::Tag("body".to_string())),
                Combinator::Child,
                Box::new(Selector::Tag("section".to_string())),
            )),
            Combinator::GeneralSibling,
            Box::new(Selector::Tag("h1".to_string())),
        );

        assert_eq!(s.to_string(), "body > section ~ h1");
    }

    #[test]
    fn pseudo_class() {
        let s = Selector::PseudoClass(
            Box::new(Selector::Tag("body".to_string())),
            "hover".to_string(),
        );

        assert_eq!(s.to_string(), "body:hover");
    }

    #[test]
    fn pseudo_element() {
        let s = Selector::PseudoElement(
            Box::new(Selector::Tag("body".to_string())),
            "first-line".to_string(),
        );

        assert_eq!(s.to_string(), "body::first-line");
    }

    #[test]
    fn attribute() {
        let s = Selector::Attribute("title".to_string());

        assert_eq!(s.to_string(), "[title]");
    }

    #[test]
    fn attribute_value() {
        let s = Selector::AttributeValue("title".to_string(), "hello".to_string());

        assert_eq!(s.to_string(), "[title=\"hello\"]");
    }

    #[test]
    fn attribute_contains() {
        let s = Selector::AttributeContains("title".to_string(), "hello".to_string());

        assert_eq!(s.to_string(), "[title~=\"hello\"]");
    }

    #[test]
    fn chain() {
        let s = Selector::Chain(vec![
            Selector::Tag("body".to_string()),
            Selector::Class("main".to_string()),
            Selector::Attribute("title".to_string()),
        ]);

        assert_eq!(s.to_string(), "body.main[title]");
    }

    #[test]
    fn group() {
        let s = Selector::Group(vec![
            Selector::Tag("body".to_string()),
            Selector::Class("main".to_string()),
            Selector::Id("title".to_string()),
        ]);

        assert_eq!(s.to_string(), "body, .main, #title");
    }

    #[test]
    fn rule() {
        let rule = Rule::new(
            Selector::Tag("body".to_string()),
            vec![
                Declaration::new(
                    "color".to_string(),
                    DeclarationValue::Basic("blue".to_string()),
                ),
                Declaration::new(
                    "background-color".to_string(),
                    DeclarationValue::Basic("red".to_string()),
                ),
                Declaration::new(
                    "font-family".to_string(),
                    DeclarationValue::Basic("Times New Roman".to_string()),
                ),
            ],
            vec![],
        );

        assert_eq!(rule.to_string(), "body{color:blue;background-color:red;font-family:\"Times New Roman\";}")
    }
}
