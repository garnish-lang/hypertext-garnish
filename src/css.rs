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
    Combinator(Selector, Combinator, Selector), // (base selector, combination)
    PseudoClass(Selector, String), // (base selector, pseudo class)
    PseudoElement(Selector, String), // (base selector, pseudo element)
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
