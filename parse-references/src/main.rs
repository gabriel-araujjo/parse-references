use std::{fmt::Write, io::Read};

use nom_bibtex::Bibtex;

use crate::abnt::Abnt;

mod abnt;

pub struct MissingTags {
    pub missing_tags: Vec<String>,
}

struct FixPunctuation<W> {
    write: W,
    last_char_type: CharType,
}

enum CharType {
    Other,
    Punctuation(char),
}

impl CharType {
    fn of(c: char) -> CharType {
        match c {
            '.' | ';' | '?' | '!' | '…' | ':' | ',' => CharType::Punctuation(c),
            _ => CharType::Other,
        }
    }
}

impl<W: std::io::Write> Write for FixPunctuation<W> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        for c in s.chars() {
            let char_type = CharType::of(c);
            match (&self.last_char_type, &char_type) {
                (CharType::Punctuation('?'), CharType::Punctuation('?'))
                | (CharType::Punctuation('?'), CharType::Punctuation('!'))
                | (CharType::Punctuation('!'), CharType::Punctuation('?'))
                | (CharType::Punctuation('!'), CharType::Punctuation('!'))
                | (CharType::Punctuation('.'), CharType::Punctuation('.'))
                | (CharType::Punctuation('.'), CharType::Punctuation(';')) => {
                    let mut buf = [0; 4];
                    self.write
                        .write(c.encode_utf8(&mut buf).as_bytes())
                        .map_err(|_| std::fmt::Error)?;
                }
                (CharType::Punctuation(_), CharType::Punctuation(_)) => {}
                _ => {
                    let mut buf = [0; 4];
                    self.write
                        .write(c.encode_utf8(&mut buf).as_bytes())
                        .map_err(|_| std::fmt::Error)?;
                }
            }
            self.last_char_type = char_type;

            if c == '\n' {
                self.write.flush().map_err(|_| std::fmt::Error)?;
            }
        }
        Ok(())
    }
}

fn main() {
    let mut buf = br#"
    @string (jan = "1")
    @string (feb = "2")
    @string (mar = "3")
    @string (apr = "4")
    @string (may = "5")
    @string (jun = "6")
    @string (jul = "7")
    @string (ago = "8")
    @string (sep = "9")
    @string (oct = "10")
    @string (nov = "11")
    @string (dec = "12")
    @string (dez = "12")
    "#
    .to_vec();
    std::io::stdin()
        .read_to_end(&mut buf)
        .expect("to read stdin");

    let buf = String::from_utf8(buf).expect("stdin is a valid utf8 string");

    let bibtex = Bibtex::parse(&buf).expect("valid bibtex");

    let mut bibs: Vec<_> = bibtex
        .bibliographies()
        .iter()
        .filter(|b| !b.citation_key().starts_with("Self"))
        .map(Abnt)
        .collect();

    bibs.sort_unstable();

    println!(
        r#"

<div class="references txt-sml txt-left proportional-nums">

## Referências

"#
    );

    let mut out = FixPunctuation {
        write: std::io::stdout(),
        last_char_type: CharType::Other,
    };

    for bib in bibs {
        writeln!(out, "{}\n", bib).expect("write to stdout");
    }

    println!("</div>");
}
