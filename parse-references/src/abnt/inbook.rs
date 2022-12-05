use std::fmt::Display;

use nom_bibtex::Bibliography;

use crate::abnt::authors::Authors;

use super::{location_publisher::LocationPublisher, uppercase::Uppercase, tex};

pub struct InBook<'i> {
    title: &'i str,
    subtitle: Option<&'i str>,
    author: &'i str,
    year: &'i str,
    publisher: &'i str,
    location: &'i str,
    bookauthor: &'i str,
    booktitle: &'i str,
    booksubtitle: Option<&'i str>,
    editor: Option<&'i str>,
}

impl<'i> InBook<'i> {
    pub fn from_bib(b: &'i Bibliography) -> Self {
        let mut book = Self {
            title: "",
            subtitle: None,
            author: "",
            location: "",
            publisher: "",
            year: "",
            bookauthor: "",
            booktitle: "",
            booksubtitle: None,
            editor: None,
        };

        for (k, v) in b.tags().iter() {
            match k.as_str() {
                "title" => book.title = v.as_str().trim(),
                "subtitle" => book.subtitle = Some(v.as_str().trim()),
                "author" => book.author = v.as_str().trim(),
                "year" => book.year = v.as_str().trim(),
                "booktitle" => book.booktitle = v.as_str().trim(),
                "booksubtitle" => book.booksubtitle = Some(v.as_str().trim()),
                "bookauthor" => book.bookauthor = v.as_str().trim(),
                "location" | "address" => book.location = v.as_str().trim(),
                "publisher" => book.publisher = v.as_str().trim(),
                "editor" => book.editor = Some(v.as_str().trim()),
                _ => continue,
            }
        }

        book
    }
}

impl<'i> Display for InBook<'i> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut authors = format!("{}", Authors(&self.author));
        if authors.ends_with(".") {
            authors.pop();
        }

        write!(f, "{}. {}", authors, self.title)?;

        if let Some(subtitle) = &self.subtitle {
            write!(f, ": {}", subtitle)?;
        }

        let mut bookauthors = format!("{}", Authors(&self.bookauthor));
        if bookauthors.ends_with(".") {
            bookauthors.pop();
        }
        
        if bookauthors.is_empty() {
            if let Some(editor) = self.editor {
                if !editor.is_empty() {
                    bookauthors = format!("{} (Org.)", Authors(editor));
                }
            }
        }

        if bookauthors.is_empty() {
            let (title_start, title_end) =
                self.booktitle.split_once(tex::match_free_char(' ')).unwrap_or((self.title, ""));
            write!(f, ". In: {} {}.", Uppercase(title_start), title_end)?;
        } else {
            write!(f, ". In: {}. <strong>{}</strong>", bookauthors, self.booktitle)?;
        }


        if let Some(booksubtitle) = &self.booksubtitle {
            write!(f, ": {}", booksubtitle)?;
        }

        let loc_pub = LocationPublisher(&*self.location, &*self.publisher);

        write!(f, ". {}, {}.", loc_pub, self.year)
    }
}


#[test]
fn simple_inbook() {
    let inbook = InBook {
        title: "A formação da economia colonial no Rio de Janeiro e de sua primeira elite senhorial (séculos XVI e XVII)",
        subtitle: None,
        author: "Fragoso, J. A.",
        year: "2001",
        publisher: "Civilização Brasileira",
        location: "Rio de Janeiro",
        bookauthor: "FRAGOSO, J. and BICALHO, M. F. and GOUVÊA, M. F.",
        booktitle: "O Antigo Regime nos trópicos",
        booksubtitle: Some("a dinâmica Imperial portuguesa (séculos XVI-XVIII)"),
        editor: None,
    };

    assert_eq!(
        format!("{inbook}"),
        "FRAGOSO, J. A. A formação da economia colonial no Rio de Janeiro e de sua primeira elite senhorial (séculos XVI e XVII). In: FRAGOSO, J.; BICALHO, M. F.; GOUVÊA, M. F. <strong>O Antigo Regime nos trópicos</strong>: a dinâmica Imperial portuguesa (séculos XVI-XVIII). Rio de Janeiro: Civilização Brasileira, 2001.",
    )
}
