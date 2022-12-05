use std::fmt::Display;

use nom_bibtex::Bibliography;

use super::date::Date;

pub struct ExtraInfo<'w> {
    url: Option<&'w str>,
    doi: Option<&'w str>,
    url_date: Option<&'w str>,
    note: Option<&'w str>,
}

impl<'w> ExtraInfo<'w> {
    pub fn from_bib(b: &'w Bibliography) -> Self {
        let mut extra_info = Self {
            url: None,
            doi: None,
            url_date: None,
            note: None,
        };

        for (k, v) in b.tags().iter() {
            match k.as_str() {
                "url" => extra_info.url = Some(v.as_str().trim()),
                "doi" => {
                    let doi = v.as_str().trim();

                    let doi = if let Some(doi) = doi.strip_prefix("https://doi.org/") {
                        doi
                    } else if let Some(doi) = doi.strip_prefix("http://doi.org/") {
                        doi
                    } else {
                        doi
                    };

                    extra_info.doi = Some(doi)
                }
                "urldate" => extra_info.url_date = Some(v.as_str().trim()),
                "note" => extra_info.note = Some(v.as_str().trim().trim_end_matches('.')),
                _ => continue,
            }
        }

        extra_info
    }
}

impl<'w> Display for ExtraInfo<'w> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(note) = self.note {
            write!(f, " {}.", note)?;
        }

        if let Some(doi) = self.doi {
            write!(
                f,
                r#" Disponível em: <span class="font-mono">&lt;<https://doi.org/{}>&gt;</span>."#,
                doi,
            )?;
        } else if let Some(u) = self.url {
            write!(
                f,
                r#" Disponível em: <span class="font-mono">&lt;<{}>&gt;</span>."#,
                u,
            )?;
        }

        if let Some(d) = self.url_date {
            write!(f, " Acesso em: {}.", Date(d))?;
        }

        Ok(())
    }
}

#[test]
fn extra_data() {
    let extra = ExtraInfo {
        url: None,
        doi: None,
        url_date: Some("2020-12-14"),
        note: Some("Some note"),
    };

    assert_eq!(format!("{}", extra), " Some note. Acesso em: 14 dez. 2020.",);
}
