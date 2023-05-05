struct Attribute {
    name: String,
    value: String,
}

struct Element {
    tag: String,
    attributes: Vec<HTMLAttribute>,
    children: Vec<HTMLElement>,
    text: String,
}