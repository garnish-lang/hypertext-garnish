struct CSSDeclaration {
    property: String,
    value: String,
}

struct CSSRule {
    selector: String,
    declarations: Vec<CSSDeclaration>,
}

struct CSSSheet {
    rules: Vec<CSSRule>,
}

