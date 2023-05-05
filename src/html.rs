struct HTMLAttribute {
    name: String,
    value: String,
}

struct HTMLElement {
    tag: String,
    attributes: Vec<HTMLAttribute>,
    children: Vec<HTMLElement>,
    text: String,
}