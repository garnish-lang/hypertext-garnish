struct Declaration {
    property: String,
    value: String,
}

enum Combinator {
    Descendant,
    Child,
    AdjacentSibling,
    GeneralSibling,
}

enum Selector {
    Tag(String), // tag name
    Class(String), // class name
    Id(String), // id name
    Combinator(Box<Selector>, Combinator, Box<Selector>), // (base selector, combination)
    PseudoClass(Box<Selector>, String), // (base selector, pseudo class)
    PseudoElement(Box<Selector>, String), // (base selector, pseudo element)
    Attribute(String), // attribute name
    AttributeValue(String, String), // (attribute name, attribute value)
    AttributeContains(String, String), // (attribute name, search string)
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
