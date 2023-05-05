struct Declaration {
    property: String,
    value: String,
}

struct Rule {
    selector: String,
    declarations: Vec<CSSDeclaration>,
}

struct RuleSet {
    media_query: Option<String>,
    rules: Vec<CSSRule>,
}

struct Sheet {
    sets: Vec<CSSSet>
}

