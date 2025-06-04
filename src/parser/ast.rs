#[derive(Debug)]
#[allow(dead_code)]
pub enum Node {
    Document(Vec<Node>),
    Heading {
        level: u8,
        content: Vec<Node>,
    },
    Paragraph(Vec<Node>),
    Bold(Vec<Node>),
    Italic(Vec<Node>),
    Text(String),
    Code {
        language: String,
        content: Vec<Node>,
    },
    LineBreak(),
    Mark(Vec<Node>),
    BlockQuote {
        r#type: u8,
        content: String,
    },
    Image {
        alt: String,
        src: String,
    },
    Table {
        headers: Vec<String>,
        rows: Vec<Vec<String>>,
    },
}

impl Node {
    pub fn new_document() -> Node {
        Node::Document(Vec::new())
    }

    pub fn push_child(&mut self, child: Node) {
        match self {
            Node::Document(children)
            | Node::Heading {
                content: children, ..
            }
            | Node::Paragraph(children)
            | Node::Bold(children)
            | Node::Italic(children) => {
                children.push(child);
            }
            _ => {
                println!("Either a text or an unrecognized");
            }
        }
    }

    pub fn to_html(&self, stylesheet: &str) -> String {
        match self {
            Node::Document(children) => {
                let inner_html: String = children
                    .iter()
                    .map(|child| child.to_html(stylesheet))
                    .collect();
                format!(
                    r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>Document</title>
    <link rel="stylesheet" href="{}"/>
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/styles/github-dark.min.css">
    <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/highlight.min.js"></script>
    <script>hljs.highlightAll();</script>
</head>
<body>
    {}
</body>
</html>"#,
                    stylesheet, inner_html
                )
            }
            Node::Heading { level, content } => {
                let inner_html: String = content
                    .iter()
                    .map(|child| child.to_html(stylesheet))
                    .collect();
                format!(
                    "<h{level}>{inner_html}</h{level}>",
                    level = level,
                    inner_html = inner_html
                )
            }
            Node::Paragraph(children) => {
                let inner_html: String = children
                    .iter()
                    .map(|child| child.to_html(stylesheet))
                    .collect();
                format!("<p>{}</p>", inner_html)
            }
            Node::Bold(children) => {
                let inner_html: String = children
                    .iter()
                    .map(|child| child.to_html(stylesheet))
                    .collect();
                format!("<strong>{}</strong>", inner_html)
            }
            Node::Italic(children) => {
                let inner_html: String = children
                    .iter()
                    .map(|child| child.to_html(stylesheet))
                    .collect();
                format!("<em>{}</em>", inner_html)
            }
            Node::Text(text) => text.to_string(),
            Node::Code {
                language: lang,
                content,
            } => {
                let inner_html: String = content
                    .iter()
                    .map(|child| child.to_html(stylesheet) + "\n")
                    .collect();
                format!(
                    "<pre><code class='language-{lang}'>{}</code>\n</pre>",
                    inner_html
                )
            }
            Node::LineBreak() => String::from("<hr>"),
            Node::BlockQuote {
                r#type: quote_type,
                content,
            } => format!(
                "<blockquote class='blockquote-{}'>{}</blockquote>",
                quote_type, content
            ),
            Node::Mark(children) => {
                let inner_html: String = children
                    .iter()
                    .map(|child| child.to_html(stylesheet))
                    .collect();
                format!("<mark>{}</mark>", inner_html)
            }
            Node::Image { alt, src } => {
                format!("<img src={} alt='{}' />", src, alt)
            }
            Node::Table { headers, rows } => {
                let thead = format!(
                    "<thead><tr>{}</tr></thead>",
                    headers
                        .iter()
                        .map(|h| format!("<th>{}</th>", h))
                        .collect::<String>()
                );

                let tbody = format!(
                    "<tbody>{}</tbody>",
                    rows.iter()
                        .map(|row| {
                            let cells = row
                                .iter()
                                .map(|cell| format!("<td>{}</td>", cell))
                                .collect::<String>();
                            format!("<tr>{}</tr>", cells)
                        })
                        .collect::<String>()
                );

                format!("<table>{}{}</table>", thead, tbody)
            }
        }
    }
}
