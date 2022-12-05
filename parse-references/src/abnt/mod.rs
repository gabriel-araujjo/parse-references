use std::{borrow::Cow, cmp::Ordering, fmt::Display};

use lazy_static::lazy_static;
use nom_bibtex::Bibliography;
use regex::{Regex, RegexBuilder};

use crate::abnt::extra::ExtraInfo;

use self::{
    article::Article, authors::Authors, book::Book, collection::Collection, inbook::InBook,
    incollection::InCollection, inproceedings::InProceedings, thesis::Thesis,
};

mod article;
mod authors;
mod book;
mod collection;
mod date;
mod extra;
mod inbook;
mod incollection;
mod inproceedings;
mod issue;
mod join;
mod location_publisher;
mod pages;
mod strong;
mod tex;
mod thesis;
mod uppercase;
mod volume;

lazy_static! {
    pub static ref AND_REGEX: Regex = RegexBuilder::new(r" and ")
        .case_insensitive(true)
        .build()
        .unwrap();
}

#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct Abnt<'b>(pub &'b Bibliography);

impl<'b> PartialOrd for Abnt<'b> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        fn find_sort_keys<'a>(b: &'a Bibliography) -> (Option<Cow<'a, str>>, Option<&'a str>) {
            let mut sort_title = None;
            let mut author = None;
            let mut editor = None;
            let mut title = None;
            let mut year = None;

            for (k, v) in b.tags() {
                match k.as_str() {
                    "sorttitle" => sort_title = Some(v),
                    "author" => author = Some(v),
                    "editor" => editor = Some(v),
                    "title" => title = Some(v),
                    "year" => year = Some(v.trim()),
                    _ => {}
                }
            }

            (
                sort_title
                    .map(|s| Cow::Borrowed(s.trim()))
                    .or_else(|| {
                        author
                            .or(editor)
                            .map(|a| Cow::Owned(format!("{}", Authors(a))))
                    })
                    .or_else(|| title.map(|t| Cow::Borrowed(t.trim()))),
                year,
            )
        }

        let (self_sort_key, self_year) = find_sort_keys(self.0);
        let (other_sort_key, other_year) = find_sort_keys(other.0);

        match self_sort_key.cmp(&other_sort_key) {
            Ordering::Equal => Some(self_year.cmp(&other_year)),
            o => Some(o),
        }
    }
}

impl<'b> Ord for Abnt<'b> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<'b> Display for Abnt<'b> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0.entry_type() {
            "article" | "online" | "movie" | "misc" => write!(f, "{}", Article::from_bib(&self.0)),
            "book" => write!(f, "{}", Book::from_bib(&self.0)),
            "thesis" => write!(f, "{}", Thesis::from_bib(&self.0)),
            "inbook" => write!(f, "{}", InBook::from_bib(&self.0)),
            "incollection" => write!(f, "{}", InCollection::from_bib(&self.0)),
            "inproceedings" => write!(f, "{}", InProceedings::from_bib(&self.0)),
            "collection" => write!(f, "{}", Collection::from_bib(&self.0)),
            _ => panic!("unexpected type: {}", self.0.entry_type()),
        }?;

        write!(f, "{}", ExtraInfo::from_bib(&self.0))
    }
}

#[test]
fn article_from_bib() {
    let entry = r"
    @article{Azevedo1959,
        title        = {Aldeias e aldeamentos},
        author       = {Azevedo, A.},
        year         = 1959,
        number       = 33,
        pages        = 27,
        journaltitle = {Boletim Paulista de Geografia}
    }
    ";

    let bibtex = nom_bibtex::Bibtex::parse(entry).expect("valid bibtex");

    for bib in bibtex.bibliographies() {
        assert_eq!(
            format!("{}", Abnt(bib)),
            "AZEVEDO, A. Aldeias e aldeamentos. <strong>Boletim Paulista de Geografia</strong>, n. 33, p. 27, 1959.",
        );
        break;
    }
}

#[test]
fn article_from_bib_3() {
    let entry = "
    @article{Chateaubriand1952Show,
        author       = {Chateaubriand, A.},
        title        = {O ``show'' de Jacques Fath},
        journal      = {Diário de Natal},
        location     = {Natal},
        pages        = {3},
        date         = {1952-07-24},
        entrysubtype = {newspaper}
      }
    ";

    let bibtex = nom_bibtex::Bibtex::parse(entry).expect("valid bibtex");

    for bib in bibtex.bibliographies() {
        assert_eq!(
            format!("{}", Abnt(bib)),
            "CHATEAUBRIAND, A. O “show” de Jacques Fath. <strong>Diário de Natal</strong>, Natal, p. 3, 24 jul. 1952.",
        );
        break;
    }
}

#[test]
fn article_from_bib_4() {
    let entry = "
    @article{DiarioDeNatal1949SemTitulo,
        journal      = {Diário de Natal},
        location     = {Natal},
        pages        = {5},
        date         = {1949-07-10},
        entrysubtype = {newspaper}
      }
    ";

    let bibtex = nom_bibtex::Bibtex::parse(entry).expect("valid bibtex");

    for bib in bibtex.bibliographies() {
        assert_eq!(
            format!("{}", Abnt(bib)),
            "<strong>Diário de Natal</strong>, Natal, p. 5, 10 jul. 1949.",
        );
        break;
    }
}

#[test]
fn article_from_bib_5() {
    let entry = "
    @online{MarizCPDOC,
        title   = {Dinarte de Medeiros Mariz | CPDOC},
        url     = {http://www.fgv.br/cpdoc/acervo/dicionarios/verbete-biografico/dinarte-de-medeiros-mariz},
        urldate = {2019-07-19}
    }
    ";

    let bibtex = nom_bibtex::Bibtex::parse(entry).expect("valid bibtex");

    for bib in bibtex.bibliographies() {
        assert_eq!(
            format!("{}", Abnt(bib)),
            r#"DINARTE de Medeiros Mariz | CPDOC.  Disponível em: <span class="font-mono">&lt;<a href="http://www.fgv.br/cpdoc/acervo/dicionarios/verbete-biografico/dinarte-de-medeiros-mariz">http://www.fgv.br/cpdoc/acervo/dicionarios/verbete-biografico/dinarte-de-medeiros-mariz</a>&gt;</span>. Acesso em: 19 jul. 2019."#,
        );
        break;
    }
}

#[test]
fn article_from_bib_6() {
    let entry = "
    @misc{RelatorioDNOC1976,
        title = {Relatóriodo Departamento Nacional de Obras Contra as Secas (DNOCS)},
        year  = {1976},
        note  = {Arquivo da Paróquia da Diocese de Caicó}
    }
    ";

    let bibtex = nom_bibtex::Bibtex::parse(entry).expect("valid bibtex");

    for bib in bibtex.bibliographies() {
        assert_eq!(
            format!("{}", Abnt(bib)),
            r#"RELATÓRIODO Departamento Nacional de Obras Contra as Secas (DNOCS). 1976. Arquivo da Paróquia da Diocese de Caicó."#,
        );
        break;
    }
}

#[test]
fn book_from_bib3() {
    let entry = "
    @movie{TroubledLand1961,
        title        = {{The troubled} land. Produção e direção de Helen Jean Rogers},
        sorttitle    = {Troubled land, The},
        location     = {Recife},
        publisher    = {ABC Studios},
        year         = {1961},
        entrysubtype = {documentary movie}
    }
    ";

    let bibtex = nom_bibtex::Bibtex::parse(entry).expect("valid bibtex");

    for bib in bibtex.bibliographies() {
        assert_eq!(
            format!("{}", Abnt(bib)),
            r#"THE TROUBLED land. Produção e direção de Helen Jean Rogers. Recife: ABC Studios, 1961."#,
        );
        break;
    }
}

#[test]
fn book_from_bib() {
    let entry = r"
    @book{Assuncao2004,
        title        = {Negócios Jesuíticos},
        author       = {Assunção, P.},
        year         = 2004,
        publisher    = {Editora da Universidade de São Paulo},
        address      = {São Paulo},
        subtitle     = {o cotidiano da administração dos bens divinos}
    }
    ";

    let bibtex = nom_bibtex::Bibtex::parse(entry).expect("valid bibtex");

    for bib in bibtex.bibliographies() {
        assert_eq!(
            format!("{}", Abnt(bib)),
            "ASSUNÇÃO, P. <strong>Negócios Jesuíticos</strong>: o cotidiano da administração dos bens divinos. São Paulo: Editora da Universidade de São Paulo, 2004.",
        );
        break;
    }
}

#[test]
fn book_from_bib2() {
    let entry = r"
    @book{Passos1854,
        author = {Passos, A. B.},
        year   = {1854}
    }
    ";

    let bibtex = nom_bibtex::Bibtex::parse(entry).expect("valid bibtex");

    for bib in bibtex.bibliographies() {
        assert_eq!(
            format!("{}", Abnt(bib)),
            "PASSOS, A. B. [s.l.: s.n.], 1854.",
        );
        break;
    }
}

#[test]
fn thesis_from_bib() {
    let entry = r"
    @thesis{Dias2011,
        title        = {Dinâmicas mercantis coloniais},
        author       = {Dias, T. A.},
        year         = 2011,
        subtitle     = {capitania do Rio Grande do Norte (1760--1821)},
        type         = {Dissertação (Mestrado em História e espaços)},
        institution  = {Universidade Federal do Rio Grande do Norte}
    }
    ";

    let bibtex = nom_bibtex::Bibtex::parse(entry).expect("valid bibtex");

    for bib in bibtex.bibliographies() {
        assert_eq!(
            format!("{}", Abnt(bib)),
            "DIAS, T. A. <strong>Dinâmicas mercantis coloniais</strong>: capitania do Rio Grande do Norte (1760–1821). 2011. Dissertação (Mestrado em História e espaços) – Universidade Federal do Rio Grande do Norte.",
        );
        break;
    }
}

#[test]
fn inbook_from_bib() {
    let entry = r"
    @inbook{Fragoso2001,
        title        = {A formação da economia colonial no Rio de Janeiro e de sua primeira elite senhorial (séculos XVI e XVII)},
        author       = {Fragoso, J. A.},
        year         = 2001,
        booktitle    = {O Antigo Regime nos trópicos},
        publisher    = {Civilização Brasileira},
        address      = {Rio de Janeiro},
        pages        = {29--71},
        bookauthor   = {FRAGOSO, J. and BICALHO, M. F. and GOUVÊA, M. F.},
        booksubtitle = {a dinâmica Imperial portuguesa (séculos XVI-XVIII)},
        edition      = 2
    }
    ";

    let bibtex = nom_bibtex::Bibtex::parse(entry).expect("valid bibtex");

    for bib in bibtex.bibliographies() {
        assert_eq!(
            format!("{}", Abnt(bib)),
            "FRAGOSO, J. A. A formação da economia colonial no Rio de Janeiro e de sua primeira elite senhorial (séculos XVI e XVII). In: FRAGOSO, J.; BICALHO, M. F.; GOUVÊA, M. F. <strong>O Antigo Regime nos trópicos</strong>: a dinâmica Imperial portuguesa (séculos XVI-XVIII). Rio de Janeiro: Civilização Brasileira, 2001.",
        );
        break;
    }
}

#[test]
fn article_from_bib_2() {
    let entry = r"
    @article{Translado1909,
        title        = {Translado do Auto de Terras do Rio Grande},
        year         = 1909,
        volume       = 7,
        number       = {1 e 2},
        pages        = {5--131},
        journaltitle = {Revista do IHGRN}
    }
    ";

    let bibtex = nom_bibtex::Bibtex::parse(entry).expect("valid bibtex");

    for bib in bibtex.bibliographies() {
        assert_eq!(
            format!("{}", Abnt(bib)),
            "TRANSLADO do Auto de Terras do Rio Grande. <strong>Revista do IHGRN</strong>, v. 7, n. 1 e 2, p. 5–131, 1909.",
        );
        break;
    }
}

#[test]
fn incollection_from_bib() {
    let entry = r"
    @incollection{Alveal2019,
        author       = {Alveal, C. M. O.},
        title        = {Uma análise preliminar das sesmarias nas Capitanias do Norte},
        pages        = {231--242},
        booktitle    = {A época moderna e o Brasil colonial},
        booksubtitle = {conceitos, fontes e pesquisas},
        editor       = {Silva, G. C. M.},
        editortype   = {organizer},
        address      = {Maceió},
        publisher    = {EDUFAL},
        year         = {2019}
      }
    ";

    let bibtex = nom_bibtex::Bibtex::parse(entry).expect("valid bibtex");

    for bib in bibtex.bibliographies() {
        assert_eq!(
            format!("{}", Abnt(bib)),
            "ALVEAL, C. M. O. Uma análise preliminar das sesmarias nas Capitanias do Norte. In: SILVA, G. C. M. (Org.). <strong>A época moderna e o Brasil colonial</strong>: conceitos, fontes e pesquisas. Maceió: EDUFAL, 2019.",
        );
        break;
    }
}

#[test]
fn inproceedings_from_bib() {
    let entry = r"
    @inproceedings{EcMOTTERTelenovela,
        author     = {Motter, Maria de Lourdes},
        title      = {Telenovela},
        subtitle   = {reflexo e refração na arte do cotidiano},
        eventtitle = {Congresso Brasileiro de Ciências da Comunicação},
        number     = {21},
        venue      = {Recife},
        eventyear  = {1998},
        location   = {Recife},
        year       = {1998},
        url        = {http://www.portcom.intercom.org.br/pdfs/de14671ff94329deb4d1756ec2696184.PDF}
      }
    ";

    let bibtex = nom_bibtex::Bibtex::parse(entry).expect("valid bibtex");

    for bib in bibtex.bibliographies() {
        assert_eq!(
            format!("{}", Abnt(bib)),
            "MOTTER, M. d. L. Telenovela: reflexo e refração na arte do cotidiano. In: CONGRESSO BRASILEIRO DE CIÊNCIAS DA COMUNICAÇÃO, 21, 1998, Recife.",
        );
        break;
    }
}

#[test]
fn collection_from_bib() {
    let entry = r"
    @collection{Lapa1980modos,
        editor     = {Lapa, J. R. A.},
        editortype = {organizer},
        title      = {Modos de produção e realidade brasileira},
        location   = {Petrópolis},
        publisher  = {Vozes},
        year       = {1980}
      }
    ";

    let bibtex = nom_bibtex::Bibtex::parse(entry).expect("valid bibtex");

    for bib in bibtex.bibliographies() {
        assert_eq!(
            format!("{}", Abnt(bib)),
            "LAPA, J. R. A. (Org.). <strong>Modos de produção e realidade brasileira</strong>. Petrópolis: Vozes, 1980.",
        );
        break;
    }
}
