use std::fmt::Display;

use nom_bibtex::Bibliography;

use super::{authors::Authors, uppercase::Uppercase, join::Join, tex};

pub struct InProceedings<'i> {
    author: &'i str,
    title: &'i str,
    subtitle: Option<&'i str>,
    eventtitle: &'i str,
    number: Option<&'i str>,
    location: &'i str,
    year: &'i str,
}

impl<'i> InProceedings<'i> {
    pub fn from_bib(b: &'i Bibliography) -> Self {
        let mut proceeding = Self {
            author: "",
            title: "",
            subtitle: None,
            eventtitle: "",
            number: None,
            location: "",
            year: "",
        };

        for (k, v) in b.tags().iter() {
            match k.as_str() {
                "title" => proceeding.title = v.as_str(),
                "subtitle" => proceeding.subtitle = Some(v.as_str()),
                "author" => proceeding.author = v.as_str(),
                "number" => proceeding.number = Some(v.as_str()),
                "year" | "eventyear" => proceeding.year = v.as_str(),
                "eventtitle" => proceeding.eventtitle = v.as_str(),
                "location" | "address" | "venue" => proceeding.location = v.as_str(),
                _ => continue,
            }
        }

        proceeding
    }
}

impl<'i> Display for InProceedings<'i> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut authors = format!("{}", Authors(&self.author));
        if authors.ends_with(".") {
            authors.pop();
        }

        write!(f, "{}. {}", authors, tex::Text(self.title))?;

        if let Some(subtitle) = &self.subtitle {
            write!(f, ": {}", tex::Text(subtitle))?;
        }

        let parts = [
            self.number,
            Some(self.year),
            Some(self.location),
        ].into_iter().filter_map(|s| s.map(tex::Text));

        write!(
            f,
            ". In: {}, {}.",
            Uppercase(self.eventtitle),
            Join::new(", ", parts),
        )
    }
}

#[test]
fn simple_inproceedings() {
    let proc = InProceedings {
        author: "Dias, P. O.",
        title: "Gentes de conquista",
        subtitle: Some("fam??lias, poder e pecu??ria na Ribeira do Apodi-Mossor?? (1676--1725)"),
        eventtitle: "Encontro Estadual de Hist??ria",
        number: Some("1"),
        location: "Guarabira, PB",
        year: "2016",
    };

    assert_eq!(
        format!("{proc}"),
        "DIAS, P. O. Gentes de conquista: fam??lias, poder e pecu??ria na Ribeira do Apodi-Mossor?? (1676???1725). In: ENCONTRO ESTADUAL DE HIST??RIA, 1, 2016, Guarabira, PB."
    )
}



/*

@inproceedings{Dias2016gentes,
  author     = {Dias, P. O.},
  title      = {Gentes de conquista},
  subtitle   = {fam??lias, poder e pecu??ria na Ribeira do Apodi-Mossor?? (1676--1725)},
  eventtitle = {Encontro Estadual de Hist??ria},
  number     = {17},
  venue      = {Guarabira, PB},
  volume     = {17},
  number     = {1},
  eventyear  = {2016},
  booktitle  = {Anais},
  location   = {Guarabira, PB},
  publisher  = {ANPUH-PB},
  year       = {2016},
  url        = {http://www.ufpb.br/evento/index.php/xviieeh/xviieeh/paper/view/3189/2709},
}

*/

#[test]
fn simple_inproceedings2() {
    let proc = InProceedings {
        author: "Motter, Maria de Lourdes",
        title: "Telenovela",
        subtitle: Some("reflexo e refra????o na arte do cotidiano"),
        eventtitle: "Congresso Brasileiro de Ci??ncias da Comunica????o",
        number: Some("21"),
        location: "Recife",
        year: "1998",
    };

    assert_eq!(
        format!("{proc}"),
        "MOTTER, M. d. L. Telenovela: reflexo e refra????o na arte do cotidiano. In: CONGRESSO BRASILEIRO DE CI??NCIAS DA COMUNICA????O, 21, 1998, Recife.",
    )
}

/*
@inproceedings{EcMOTTERTelenovela,
  author     = {Motter, Maria de Lourdes},
  title      = {Telenovela},
  subtitle   = {reflexo e refra????o na arte do cotidiano},
  eventtitle = {Congresso Brasileiro de Ci??ncias da Comunica????o},
  number     = {21},
  venue      = {Recife},
  eventyear  = {1998},
  location   = {Recife},
  year       = {1998},
  url        = {http://www.portcom.intercom.org.br/pdfs/de14671ff94329deb4d1756ec2696184.PDF}
}
*/
