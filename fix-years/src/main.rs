use nom_bibtex::{Bibliography, Bibtex};
use std::{env::args, io::Read};

fn main() {
    let mut buf = Vec::new();
    std::io::stdin()
        .read_to_end(&mut buf)
        .expect("to read stdin");

    let prefix = args().nth(1).expect("prefix arg");

    let buf = String::from_utf8(buf).expect("stdin is a valid utf8 string");

    let bibtex = Bibtex::parse(&buf).expect("valid bibtex");

    for bib in bibtex.bibliographies().iter() {
        print_bib(&prefix, bib)
    }
}

fn print_bib(prefix: &str, bib: &Bibliography) {
    let mut citation_key = prefix.to_owned();
    citation_key += bib.citation_key();

    let year = bib
        .tags()
        .iter()
        .find(|(k, _)| *k == "year")
        .map(|(_, v)| v);

    if let Some(year) = year {
        citation_key = citation_key.replace("2021", &year);
    }

    let entry_type = bib.entry_type();

    let width = bib
        .tags()
        .iter()
        .map(|(k, _)| k.len())
        .fold(0, std::cmp::max)
        + 1;

    print!("@{entry_type}{{{citation_key}");

    for (k, v) in bib.tags() {
        print!(",\n  {k:width$}= {{{v}}}");
    }

    println!("\n}}");
}
