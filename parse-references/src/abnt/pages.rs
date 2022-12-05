use std::fmt::Display;

pub enum Pages<'p> {
    Single(&'p str),
    Range(&'p str, &'p str),
}

impl<'p> Display for Pages<'p> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pages::Single(page) => write!(f, "p. {page}"),
            Pages::Range(first, last) => write!(f, "p. {first}–{last}"),
        }
    }
}

impl<'p> Pages<'p> {
    pub fn from_str(s: &'p str) -> Self {
        let mut parts = s.split("--");
        
        let first = parts.next().unwrap_or("");
        let last = parts.next();

        if let Some(last) = last {
            Self::Range(first, last)
        } else {
            Self::Single(first)
        }
    }
}

#[test]
fn single_page() {
    let p = Pages::Single("v");
    let output = format!("{p}");
    
    assert_eq!(output, "p. v");
}

#[test]
fn page_range() {
    let p = Pages::Range("2", "10");
    let output = format!("{p}");
    
    assert_eq!(output, "p. 2–10");
}
