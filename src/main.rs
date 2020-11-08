use anyhow::{anyhow, Result};
use std::io::{stdin, stdout, BufWriter, Read, Write};
use std::write;

const FOOTNOTE_START: &str = "[{";
const FOOTNOTE_END: &str = "}]";

fn main() -> Result<()> {
    let mut content = String::new();
    stdin().read_to_string(&mut content)?;

    let mut out = BufWriter::new(stdout());

    #[derive(Copy, Clone, Debug)]
    enum ContentType {
        Footnote,
        NotFootnote,
    }

    let wrap_footnote_link = |index| {
        format!(
            "<sup id=\"fnref:{}\"><a href=\"#fn:{}\" rel=\"footnote\">{}</a></sup>",
            index, index, index
        )
    };
    let mut footnotes = vec![];
    for (i, section) in content.split(FOOTNOTE_START).enumerate() {
        let mut split = section.split(FOOTNOTE_END);

        let content = split.next().unwrap_or("");
        let (footnote, content) = match split.next() {
            Some(after_footnote) => (content, after_footnote),
            None => {
                write!(out, "{}", content);
                continue;
            }
        };

        let index = footnotes.len() + 1;
        write!(out, "{}", wrap_footnote_link(index));
        write!(out, "{}", content);
        footnotes.push(footnote);
    }

    if footnotes.is_empty() {
        return Ok(());
    }

    write!(out, "\n\n<aside class=\"footnotes\"><ol>");
    for (index, footnote) in footnotes.into_iter().enumerate() {
        let index = index + 1;
        write!(
            out,
            "<li class=\"footnote\" id=\"fn:{}\"><p>{}<a href=\"#fnref:{}\"></a></p></li>",
            index, footnote, index
        );
    }
    write!(out, "</ol></aside>");

    Ok(())
}
