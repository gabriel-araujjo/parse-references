use std::fmt::Display;

use nom_bibtex::Bibliography;

use super::{
    authors::Authors, date::Date, issue::Issue, join::Join, pages::Pages, strong::Strong, tex,
    uppercase::Uppercase, volume::Volume, location_publisher::LocationPublisher,
};

pub struct Article<'a> {
    author: &'a str,
    title: &'a str,
    subtitle: Option<&'a str>,
    journal: Option<&'a str>,
    location: Option<&'a str>,
    publisher: Option<&'a str>,
    issue: Option<&'a str>,
    volume: Option<&'a str>,
    pages: Option<Pages<'a>>,
    year: Option<&'a str>,
    date: Option<&'a str>,
}

impl<'a> Article<'a> {
    pub fn from_bib(b: &'a Bibliography) -> Self {
        let mut article = Article {
            author: "",
            title: "",
            subtitle: None,
            journal: None,
            location: None,
            publisher: None,
            issue: None,
            volume: None,
            pages: None,
            year: None,
            date: None,
        };

        for (k, v) in b.tags().iter() {
            match k.as_str() {
                "author" => article.author = v.trim(),
                "title" => article.title = v.trim(),
                "subtitle" => article.subtitle = Some(v.trim()),
                "journal" | "journaltitle" => article.journal = Some(v.trim()),
                "location" | "address" => article.location = Some(v.trim()),
                "publisher" => article.publisher = Some(v.trim()),
                "issue" | "number" => article.issue = Some(v.trim()),
                "volume" => article.volume = Some(v.trim()),
                "page" | "pages" => article.pages = Some(Pages::from_str(v.trim())),
                "year" => article.year = Some(v.trim()),
                "date" => article.date = Some(v.trim()),
                _ => continue,
            }
        }

        article
    }
}

fn as_dyn_display<D: Display>(d: &D) -> &dyn Display {
    d
}

impl<'a> Display for Article<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut authors = if self.author.is_empty() {
            String::new()
        } else {
            format!("{}", Authors(&self.author))
        };

        if authors.ends_with(".") {
            authors.pop();
        }

        let journal = self.journal.map(|j| Strong(tex::Text(j)));

        let issue = self.issue.map(Issue);

        let volume = self.volume.map(Volume);

        let mut date = None;

        let loc_pub = Some(LocationPublisher(
            self.location.unwrap_or(""),
            self.publisher.unwrap_or(""),
        ));

        let loc_pub = if self.publisher.is_some() {
            loc_pub.as_ref().map(as_dyn_display)
        } else {
            self.location.as_ref().map(as_dyn_display)
        };

        let mut parts = [
            journal.as_ref().map(as_dyn_display),
            loc_pub,
            volume.as_ref().map(as_dyn_display),
            issue.as_ref().map(as_dyn_display),
            self.pages.as_ref().map(as_dyn_display),
            self.year.as_ref().map(as_dyn_display).or_else(|| {
                date = self.date.map(Date);
                date.as_ref().map(as_dyn_display)
            }),
        ]
        .into_iter()
        .filter_map(|i| i).peekable();

        let (title_start, title_end) = if self.author.is_empty() {
            self.title
                .split_once(tex::match_free_char(' '))
                .map(|(s, e)| (s, Some(e)))
                .unwrap_or((self.title, None))
        } else {
            ("", Some(self.title))
        };

        let title = [title_end, self.subtitle]
            .into_iter()
            .filter_map(|i| i.map(tex::Text));

        if self.author.is_empty() {
            if !title_start.is_empty() {
                write!(f, "{} ", Uppercase(title_start))?;
            }
        } else {
            write!(f, "{}. ", authors)?;
        }

        if title_end.is_some() || self.subtitle.is_some() {
            write!(f, "{}. ", Join::new(": ", title))?;
        }

        if parts.peek().is_some() {
            write!(f, "{}.", Join::new(", ", parts),)?;
        }

        Ok(())
    }
}

#[test]
fn simple_article() {
    /*
     author   = {Rezende, M. J.},
       title    = {Os sertões e os (des)caminhos da mudança social no Brasil},
       location = {São Paulo},
       journal  = {Tempo Social: Revista de Sociologia da USP},
       volume   = {13},
       number   = {2},
       year     = {2001},
       pages    = {201--226}
    */

    let article = Article {
        author: "Rezende, M. J.",
        title: "Os sertões e os (des)caminhos da mudança social no Brasil",
        subtitle: None,
        journal: Some("Tempo Social: Revista de Sociologia da USP"),
        location: Some("São Paulo"),
        publisher: None,
        issue: Some("2"),
        volume: Some("13"),
        pages: Some(Pages::Range("201", "226")),
        year: Some("2001"),
        date: None,
    };

    let output = format!("{}", article);

    assert_eq!(
        output,
        "REZENDE, M. J. Os sertões e os (des)caminhos da mudança social no Brasil. <strong>Tempo Social: Revista de Sociologia da USP</strong>, São Paulo, v. 13, n. 2, p. 201–226, 2001."
    );
}
