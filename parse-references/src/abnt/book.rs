use std::fmt::Display;

use nom_bibtex::Bibliography;

use crate::abnt::{authors::Authors, location_publisher::LocationPublisher};

use super::tex;

pub struct Book<'b> {
    title: &'b str,
    subtitle: Option<&'b str>,
    author: &'b str,
    editor: bool,
    year: &'b str,
    location: &'b str,
    publisher: &'b str,
}

impl<'a> Book<'a> {
    pub fn from_bib(b: &'a Bibliography) -> Self {
        let mut book = Book {
            title: "",
            subtitle: None,
            author: "",
            editor: false,
            location: "",
            publisher: "",
            year: "",
        };

        let mut wrong_editor_type = true;

        for (k, v) in b.tags().iter() {
            match k.as_str() {
                "author" => book.author = v.as_str(),
                "editor" => {
                    book.author = v.as_str();
                    book.editor = true;
                }
                "organizer" => {
                    book.author = v.as_str();
                    book.editor = true;
                    wrong_editor_type = false;
                }
                "editortype" => {
                    if v == "organizer" {
                        wrong_editor_type = false;
                    }
                }
                "title" => book.title = v.as_str(),
                "subtitle" => book.subtitle = Some(v.as_str()),
                "location" | "address" => book.location = v.as_str(),
                "publisher" => book.publisher = v.as_str(),
                "year" => book.year = v.as_str(),
                _ => continue,
            }
        }

        if book.editor && wrong_editor_type {
            panic!("invalid editor type: expecting organizer, citation_key: {}", b.citation_key())
        }

        book
    }
}

impl<'b> Display for Book<'b> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.editor {
            write!(f, "{} (Org.). ", Authors(self.author))?;
        } else {
            let mut authors = format!("{}", Authors(&self.author));
            if authors.ends_with(".") {
                authors.pop();
            }
            write!(f, "{}. ", authors)?;
        }

        if !self.title.is_empty() {
            write!(f, "<strong>{}</strong>", tex::Text(self.title))?;

            if let Some(subtitle) = self.subtitle {
                write!(f, ": {}", tex::Text(subtitle))?;
            }

            f.write_str(". ")?;
        }

        write!(
            f,
            "{}, {}.",
            LocationPublisher(&self.location, &self.publisher),
            self.year,
        )
    }
}

#[test]
fn abreu() {
    let book = Book {
        title: "Caminhos antigos e povoamento do Brasil",
        subtitle: None,
        author: "Abreu, J. C. d.",
        editor: false,
        year: "1988",
        location: "Belo Horizonte AND São Paulo",
        publisher: "Itatiaia AND EDUSP",
    };

    assert_eq!(format!("{book}"), "ABREU, J. C. d. <strong>Caminhos antigos e povoamento do Brasil</strong>. Belo Horizonte: Itatiaia; São Paulo: EDUSP, 1988.")
}

#[test]
fn ackerman() {
    let book = Book {
        title: "Uma História Natural dos sentidos",
        subtitle: None,
        editor: false,
        author: "Ackerman, D.",
        year: "1990",
        location: "Rio de Janeiro",
        publisher: "Bertrand Brasil",
    };

    assert_eq!(
        format!("{book}"),
        "ACKERMAN, D. <strong>Uma História Natural dos sentidos</strong>. Rio de Janeiro: Bertrand Brasil, 1990.",
    )
}
