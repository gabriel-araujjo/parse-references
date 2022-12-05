use std::fmt::Display;

use nom_bibtex::Bibliography;

use super::{authors::Authors, location_publisher::LocationPublisher};

pub struct InCollection<'i> {
    author: &'i str,
    title: &'i str,
    subtitle: Option<&'i str>,
    booktitle: &'i str,
    booksubtitle: Option<&'i str>,
    editor: &'i str,
    location: &'i str,
    publisher: &'i str,
    year: &'i str,
}

impl<'i> InCollection<'i> {
    pub fn from_bib(b: &'i Bibliography) -> Self {
        let mut collection = Self {
            title: "",
            subtitle: None,
            author: "",
            location: "",
            publisher: "",
            year: "",
            editor: "",
            booktitle: "",
            booksubtitle: None,
        };

        let mut wrong_editor_type = true;

        for (k, v) in b.tags().iter() {
            match k.as_str() {
                "title" => collection.title = v.as_str(),
                "subtitle" => collection.subtitle = Some(v.as_str()),
                "author" => collection.author = v.as_str(),
                "year" => collection.year = v.as_str(),
                "booktitle" => collection.booktitle = v.as_str(),
                "booksubtitle" => collection.booksubtitle = Some(v.as_str()),
                "editor" => collection.editor = v.as_str(),
                "organizer" => {
                    collection.editor = v.as_str();
                    wrong_editor_type = false;
                }
                "location" | "address" => collection.location = v.as_str(),
                "publisher" => collection.publisher = v.as_str(),
                "editortype" => {
                    if v == "organizer" {
                        wrong_editor_type = false;
                    }
                }
                _ => continue,
            }
        }

        if wrong_editor_type {
            panic!(
                "invalid editor type: expecting organizer, citation_key: {}",
                b.citation_key()
            )
        }

        collection
    }
}

impl<'i> Display for InCollection<'i> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut authors = format!("{}", Authors(&self.author));
        if authors.ends_with(".") {
            authors.pop();
        }

        write!(f, "{}. {}", authors, self.title)?;

        if let Some(subtitle) = &self.subtitle {
            write!(f, ": {}", subtitle)?;
        }

        write!(
            f,
            ". In: {} (Org.). <strong>{}</strong>",
            Authors(&self.editor),
            self.booktitle
        )?;

        if let Some(booksubtitle) = &self.booksubtitle {
            write!(f, ": {}", booksubtitle)?;
        }

        let loc_pub = LocationPublisher(&*self.location, &*self.publisher);

        write!(f, ". {}, {}.", loc_pub, self.year)
    }
}

#[test]
fn simple_incollection() {
    let incollection = InCollection {
        author: "Alveal, C. M. O.",
        title: "Uma análise preliminar das sesmarias nas Capitanias do Norte",
        subtitle: None,
        booktitle: "A época moderna e o Brasil colonial",
        booksubtitle: Some("conceitos, fontes e pesquisas"),
        editor: "Silva, G. C. M.",
        location: "Maceió",
        publisher: "EDUFAL",
        year: "2019",
    };

    assert_eq!(
        format!("{incollection}"),
        "ALVEAL, C. M. O. Uma análise preliminar das sesmarias nas Capitanias do Norte. In: SILVA, G. C. M. (Org.). <strong>A época moderna e o Brasil colonial</strong>: conceitos, fontes e pesquisas. Maceió: EDUFAL, 2019.",
    )
}
