use std::fmt::Display;

use nom_bibtex::Bibliography;

use crate::abnt::authors::Authors;

use super::{tex, location_publisher::LocationPublisher};

pub struct Collection<'c> {
    editor: &'c str,
    title: &'c str,
    subtitle: Option<&'c str>,
    location: &'c str,
    publisher: &'c str,
    year: &'c str,
}

impl<'a> Collection<'a> {
    pub fn from_bib(b: &'a Bibliography) -> Self {
        let mut collection = Self {
            title: "",
            subtitle: None,
            editor: "",
            location: "",
            publisher: "",
            year: "",
        };

        let mut wrong_editor_type = true;

        for (k, v) in b.tags().iter() {
            match k.as_str() {
                "editor" => collection.editor = v.as_str(),
                "editortype" => if v == "organizer" {
                    wrong_editor_type = false;
                }
                "organizer" => {
                    collection.editor = v.as_str();
                    wrong_editor_type = false;
                }
                "title" => collection.title = v.as_str(),
                "subtitle" => collection.subtitle = Some(v.as_str()),
                "location" | "address" => collection.location = v.as_str(),
                "publisher" => collection.publisher = v.as_str(),
                "year" => collection.year = v.as_str(),
                _ => continue,
            }
        }

        if wrong_editor_type {
            panic!("invalid editor type: expecting organizer")
        }

        collection
    }
}

impl<'c> Display for Collection<'c> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} (Org.). <strong>{}</strong>",
            Authors(&self.editor),
            tex::Text(self.title),
        )?;

        if let Some(subtitle) = self.subtitle {
            write!(f, ": {}", tex::Text(subtitle))?;
        }

        write!(
            f,
            ". {}, {}.",
            LocationPublisher(&self.location, &self.publisher),
            self.year,
        )
    }
}

#[test]
fn simple_collection() {
    let col = Collection {
        editor: "Lapa, J. R. A.",
        title: "Modos de produção e realidade brasileira",
        subtitle: None,
        location: "Petrópolis",
        publisher: "Vozes",
        year: "1980",
    };

    assert_eq!(
        format!("{col}"),
        "LAPA, J. R. A. (Org.). <strong>Modos de produção e realidade brasileira</strong>. Petrópolis: Vozes, 1980."
    )
}
