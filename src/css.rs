struct Declaration {
    property: String,
    value: String,
}

struct Rule {
    selector: String,
    declarations: Vec<Declaration>,
    sub_rules: Vec<Rule>,
}

struct RuleSet {
    media_query: Option<String>,
    rules: Vec<Rule>,
    sub_sets: Vec<RuleSet>,
}
