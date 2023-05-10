use serde::Deserialize;

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
pub enum DeclarationValue {
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
            DeclarationValue::Function(name, args) => format!("{}({})", name, args.join(",")),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
pub struct Declaration {
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

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
pub enum Combinator {
    Descendant,
    Child,
    AdjacentSibling,
    GeneralSibling,
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
pub enum Selector {
    Universal,
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
            Selector::Universal => "*".to_string(),
            Selector::Tag(s) => s.to_string(),
            Selector::Id(id) => format!("#{}", id),
            Selector::Class(class) => format!(".{}", class),
            Selector::Combinator(base, op, relative) => {
                format!(
                    "{}{}{}",
                    base.to_string(),
                    match op {
                        Combinator::Descendant => " ",
                        Combinator::Child => ">",
                        Combinator::AdjacentSibling => "+",
                        Combinator::GeneralSibling => "~",
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
                .join(","),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
pub struct Rule {
    selector: Selector,
    declarations: Vec<Declaration>,
    #[serde(default)]
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

    fn make_string(&self) -> String {
        let mut all_rules = vec![format!(
            "{}{{{}}}",
            self.selector.to_string(),
            self.declarations
                .iter()
                .map(Declaration::to_string)
                .collect::<Vec<String>>()
                .join("")
        )];

        let mut sub_rules = vec![(format!("{}>", self.selector.to_string()), &self.sub_rules)];

        while let Some((prefix, rules)) = sub_rules.pop() {
            for rule in rules {
                all_rules.push(format!(
                    "{}{}{{{}}}",
                    prefix,
                    rule.selector.to_string(),
                    rule.declarations
                        .iter()
                        .map(Declaration::to_string)
                        .collect::<Vec<String>>()
                        .join("")
                ));

                if !rule.sub_rules.is_empty() {
                    sub_rules.push((
                        format!("{}{}>", prefix, rule.selector.to_string()),
                        &rule.sub_rules,
                    ))
                }
            }
        }

        all_rules.join("")
    }
}

impl ToString for Rule {
    fn to_string(&self) -> String {
        self.make_string()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
pub enum MediaConstraint {
    None,
    Not,
    Only,
}

impl Default for MediaConstraint {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
pub struct MediaFeature {
    property: String,
    value: String,
}

impl MediaFeature {
    pub fn new(property: String, value: String) -> Self {
        Self { property, value }
    }
}

impl ToString for MediaFeature {
    fn to_string(&self) -> String {
        format!("({}:{})", self.property, self.value)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
pub enum MediaCondition {
    Lone(MediaFeature),
    And(MediaFeature, MediaFeature),
    Or(MediaFeature, MediaFeature),
    Not(MediaFeature, MediaFeature),
}

impl ToString for MediaCondition {
    fn to_string(&self) -> String {
        match self {
            MediaCondition::Lone(f) => f.to_string(),
            MediaCondition::And(f1, f2) => format!("{} and {}", f1.to_string(), f2.to_string()),
            MediaCondition::Or(f1, f2) => format!("{} or {}", f1.to_string(), f2.to_string()),
            MediaCondition::Not(f1, f2) => format!("{} not {}", f1.to_string(), f2.to_string())
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
pub struct MediaQuery {
    media_type: String,
    #[serde(default)]
    constraint: MediaConstraint,
    #[serde(default)]
    features: Vec<MediaCondition>,
}

impl MediaQuery {
    pub fn new(
        constraint: MediaConstraint,
        media_type: String,
        features: Vec<MediaCondition>,
    ) -> Self {
        Self {
            media_type,
            constraint,
            features,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
pub struct RuleSet {
    media_query: Option<MediaQuery>,
    rules: Vec<Rule>,
    #[serde(default)]
    sub_sets: Vec<RuleSet>,
}

impl RuleSet {
    pub fn new(rules: Vec<Rule>, sub_sets: Vec<RuleSet>, media_query: Option<MediaQuery>) -> Self {
        Self {
            rules,
            sub_sets,
            media_query,
        }
    }
}

impl ToString for RuleSet {
    fn to_string(&self) -> String {
        let all_sets = format!(
            "{}{}",
            self.rules
                .iter()
                .map(Rule::to_string)
                .collect::<Vec<String>>()
                .join(""),
            self.sub_sets
                .iter()
                .map(RuleSet::to_string)
                .collect::<Vec<String>>()
                .join(""),
        );

        match &self.media_query {
            None => all_sets,
            Some(query) => format!(
                "@media {}{}{}{{{}}}",
                match query.constraint {
                    MediaConstraint::None => "",
                    MediaConstraint::Only => "only ",
                    MediaConstraint::Not => "not ",
                },
                query.media_type,
                match query.features.len() {
                    0 => String::new(),
                    _ => format!(
                        " and {}",
                        query
                            .features
                            .iter()
                            .map(MediaCondition::to_string)
                            .collect::<Vec<String>>()
                            .join("")
                    ),
                },
                all_sets
            ),
        }
    }
}

#[cfg(test)]
mod to_string {
    use crate::css::{
        Combinator, Declaration, DeclarationValue, MediaCondition, MediaConstraint, MediaFeature,
        MediaQuery, Rule, RuleSet, Selector,
    };

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
        assert_eq!(d.to_string(), "color:rgb(200,200,200);")
    }

    #[test]
    fn universal_selector() {
        let s = Selector::Universal;

        assert_eq!(s.to_string(), "*");
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

        assert_eq!(s.to_string(), "body>h1");
    }

    #[test]
    fn combinator_adjacent_sibling() {
        let s = Selector::Combinator(
            Box::new(Selector::Tag("body".to_string())),
            Combinator::AdjacentSibling,
            Box::new(Selector::Tag("h1".to_string())),
        );

        assert_eq!(s.to_string(), "body+h1");
    }

    #[test]
    fn combinator_general_sibling() {
        let s = Selector::Combinator(
            Box::new(Selector::Tag("body".to_string())),
            Combinator::GeneralSibling,
            Box::new(Selector::Tag("h1".to_string())),
        );

        assert_eq!(s.to_string(), "body~h1");
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

        assert_eq!(s.to_string(), "body>section~h1");
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

        assert_eq!(s.to_string(), "body,.main,#title");
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

        assert_eq!(
            rule.to_string(),
            "body{color:blue;background-color:red;font-family:\"Times New Roman\";}"
        )
    }

    #[test]
    fn rule_with_sub_rules() {
        let rule = Rule::new(
            Selector::Tag("body".to_string()),
            vec![Declaration::new(
                "color".to_string(),
                DeclarationValue::Basic("blue".to_string()),
            )],
            vec![Rule::new(
                Selector::Tag("section".to_string()),
                vec![Declaration::new(
                    "background-color".to_string(),
                    DeclarationValue::Basic("red".to_string()),
                )],
                vec![Rule::new(
                    Selector::Tag("h1".to_string()),
                    vec![Declaration::new(
                        "font-family".to_string(),
                        DeclarationValue::Basic("Times New Roman".to_string()),
                    )],
                    vec![],
                )],
            )],
        );

        assert_eq!(
            rule.to_string(),
            "body{color:blue;}body>section{background-color:red;}body>section>h1{font-family:\"Times New Roman\";}"
        )
    }

    fn make_rule_set() -> RuleSet {
        RuleSet::new(
            vec![
                Rule::new(
                    Selector::Tag("body".to_string()),
                    vec![Declaration::new(
                        "color".to_string(),
                        DeclarationValue::Basic("blue".to_string()),
                    )],
                    vec![],
                ),
                Rule::new(
                    Selector::Tag("section".to_string()),
                    vec![Declaration::new(
                        "background-color".to_string(),
                        DeclarationValue::Basic("red".to_string()),
                    )],
                    vec![],
                ),
                Rule::new(
                    Selector::Tag("h1".to_string()),
                    vec![Declaration::new(
                        "font-family".to_string(),
                        DeclarationValue::Basic("Times New Roman".to_string()),
                    )],
                    vec![],
                ),
            ],
            vec![],
            None,
        )
    }

    #[test]
    fn rule_set() {
        let set = make_rule_set();

        assert_eq!(
            set.to_string(),
            "body{color:blue;}section{background-color:red;}h1{font-family:\"Times New Roman\";}"
        )
    }

    #[test]
    fn rule_set_with_query() {
        let mut set = make_rule_set();
        set.media_query = Some(MediaQuery::new(
            MediaConstraint::None,
            "screen".to_string(),
            vec![],
        ));

        assert_eq!(
            set.to_string(),
            "@media screen{body{color:blue;}section{background-color:red;}h1{font-family:\"Times New Roman\";}}"
        )
    }

    #[test]
    fn rule_set_with_query_constraint_only() {
        let mut set = make_rule_set();
        set.media_query = Some(MediaQuery::new(
            MediaConstraint::Only,
            "screen".to_string(),
            vec![],
        ));

        assert_eq!(
            set.to_string(),
            "@media only screen{body{color:blue;}section{background-color:red;}h1{font-family:\"Times New Roman\";}}"
        )
    }

    #[test]
    fn rule_set_with_query_constraint_not() {
        let mut set = make_rule_set();
        set.media_query = Some(MediaQuery::new(
            MediaConstraint::Not,
            "screen".to_string(),
            vec![],
        ));

        assert_eq!(
            set.to_string(),
            "@media not screen{body{color:blue;}section{background-color:red;}h1{font-family:\"Times New Roman\";}}"
        )
    }

    #[test]
    fn rule_set_with_query_with_feature() {
        let mut set = make_rule_set();
        set.media_query = Some(MediaQuery::new(
            MediaConstraint::None,
            "screen".to_string(),
            vec![MediaCondition::Lone(MediaFeature::new(
                "max-width".to_string(),
                "1000px".to_string(),
            ))],
        ));

        assert_eq!(
            set.to_string(),
            "@media screen and (max-width:1000px){body{color:blue;}section{background-color:red;}h1{font-family:\"Times New Roman\";}}"
        )
    }

    #[test]
    fn rule_set_with_query_with_and_feature() {
        let mut set = make_rule_set();
        set.media_query = Some(MediaQuery::new(
            MediaConstraint::None,
            "screen".to_string(),
            vec![MediaCondition::And(
                MediaFeature::new("max-width".to_string(), "1000px".to_string()),
                MediaFeature::new("orientation".to_string(), "landscape".to_string()),
            )],
        ));

        assert_eq!(
            set.to_string(),
            "@media screen and (max-width:1000px) and (orientation:landscape){body{color:blue;}section{background-color:red;}h1{font-family:\"Times New Roman\";}}"
        )
    }

    #[test]
    fn rule_set_with_query_with_or_feature() {
        let mut set = make_rule_set();
        set.media_query = Some(MediaQuery::new(
            MediaConstraint::None,
            "screen".to_string(),
            vec![MediaCondition::Or(
                MediaFeature::new("max-width".to_string(), "1000px".to_string()),
                MediaFeature::new("orientation".to_string(), "landscape".to_string()),
            )],
        ));

        assert_eq!(
            set.to_string(),
            "@media screen and (max-width:1000px) or (orientation:landscape){body{color:blue;}section{background-color:red;}h1{font-family:\"Times New Roman\";}}"
        )
    }

    #[test]
    fn rule_set_with_query_with_not_feature() {
        let mut set = make_rule_set();
        set.media_query = Some(MediaQuery::new(
            MediaConstraint::None,
            "screen".to_string(),
            vec![MediaCondition::Not(
                MediaFeature::new("max-width".to_string(), "1000px".to_string()),
                MediaFeature::new("orientation".to_string(), "landscape".to_string()),
            )],
        ));

        assert_eq!(
            set.to_string(),
            "@media screen and (max-width:1000px) not (orientation:landscape){body{color:blue;}section{background-color:red;}h1{font-family:\"Times New Roman\";}}"
        )
    }

    #[test]
    fn rule_set_multiple_no_media_query_dont_nest() {
        let mut set = make_rule_set();
        set.sub_sets.push(make_rule_set());

        assert_eq!(set.to_string(), "body{color:blue;}section{background-color:red;}h1{font-family:\"Times New Roman\";}body{color:blue;}section{background-color:red;}h1{font-family:\"Times New Roman\";}")
    }

    #[test]
    fn rule_set_multiple_with_media_query() {
        let mut set = make_rule_set();
        let mut with_media = make_rule_set();
        with_media.media_query = Some(MediaQuery::new(
            MediaConstraint::None,
            "screen".to_string(),
            vec![],
        ));
        set.sub_sets.push(with_media);

        assert_eq!(set.to_string(), "body{color:blue;}section{background-color:red;}h1{font-family:\"Times New Roman\";}@media screen{body{color:blue;}section{background-color:red;}h1{font-family:\"Times New Roman\";}}")
    }
}
