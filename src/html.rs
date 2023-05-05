struct Attribute {
    name: String,
    value: String,
}

struct Element {
    tag: String,
    attributes: Vec<Attribute>,
    children: Vec<Element>,
    text: String,
}