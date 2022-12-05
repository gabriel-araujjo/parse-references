use std::fmt::{Display, Write};

use nom_bibtex::Bibliography;

use super::{authors::Authors, tex};

pub struct Thesis<'t> {
    author: &'t str,
    title: &'t str,
    subtitle: Option<&'t str>,
    thesis_type: &'t str,
    institution: &'t str,
    location: Option<&'t str>,
    year: &'t str,
}

impl<'t> Thesis<'t> {
    pub fn from_bib(b: &'t Bibliography) -> Self {
        let mut thesis = Thesis {
            author: "",
            title: "",
            subtitle: None,
            thesis_type: "",
            institution: "",
            location: None,
            year: "",
        };

        for (k, v) in b.tags().iter() {
            match k.as_str() {
                "author" => thesis.author = v.as_str(),
                "title" => thesis.title = v.as_str(),
                "subtitle" => thesis.subtitle = Some(v.as_str()),
                "type" => thesis.thesis_type = v.as_str(),
                "institution" => thesis.institution = v.as_str(),
                "location" | "address" => thesis.location = Some(v.as_str()),
                "year" => thesis.year = v.as_str(),
                _ => continue,
            }
        }

        thesis
    }
}

impl<'t> Display for Thesis<'t> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut authors = format!("{}", Authors(&self.author));
        if authors.ends_with(".") {
            authors.pop();
        }

        write!(f, "{}. <strong>{}</strong>", authors, tex::Text(self.title))?;

        if let Some(subtitle) = self.subtitle.as_ref() {
            write!(f, ": {}", tex::Text(subtitle))?;
        }

        write!(
            f,
            ". {}. {} – {}",
            self.year,
            tex::Text(self.thesis_type),
            tex::Text(self.institution),
        )?;

        if let Some(loc) = self.location {
            write!(f, ", {}", loc)?;
        }

        f.write_char('.')
    }
}

#[test]
fn simple_thesis() {
    let thesis = Thesis {
        title: "Onde fica o sertão rompem-se as águas",
        author: "Dias, P. O.",
        year: "2015",
        subtitle: Some("processo de territorialização da ribeira do Apodi-Mossoró (1676–1725)"),
        thesis_type: "Dissertação (Mestrado em História)",
        institution: "Universidade Federal do Rio Grande do Norte",
        location: Some("Natal"),
    };

    assert_eq!(
        format!("{thesis}"),
        "DIAS, P. O. <strong>Onde fica o sertão rompem-se as águas</strong>: processo de territorialização da ribeira do Apodi-Mossoró (1676–1725). 2015. Dissertação (Mestrado em História) – Universidade Federal do Rio Grande do Norte, Natal."
    )
}
