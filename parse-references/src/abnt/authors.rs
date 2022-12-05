use std::fmt::Display;

use crate::abnt::join::Join;

use super::{tex, uppercase::Uppercase, AND_REGEX};

#[repr(transparent)]
pub struct Authors<'a>(pub &'a str);

#[repr(transparent)]
pub struct SurnameFirst<'a>(pub &'a str);

impl<'a> Display for Authors<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.is_empty() {
            return Ok(());
        }

        let authors: Vec<_> = AND_REGEX.split(self.0).collect();

        let mut iter = authors.iter();

        if let Some(author) = iter.next() {
            write!(f, "{}", SurnameFirst(author.trim()))?;
        } else {
            return Ok(());
        }

        if authors.len() >= 4 {
            write!(f, "; <em>et al</em>")?;
        } else {
            for author in iter {
                write!(f, "; {}", SurnameFirst(*author))?;
            }
        }

        Ok(())
    }
}

impl<'a> Display for SurnameFirst<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (family, given) = if let Some(pair) = self.0.split_once(tex::match_free_char(',')) {
            pair
        } else {
            if let Some(i) = self.0.rfind(tex::match_free_char(' ')) {
                self.0.split_at(i)
            } else {
                return write!(f, "{}", Uppercase(self.0));
            }
        };

        let mut family_parts = family.split_whitespace().peekable();

        let mut extra_given_parts = Vec::new();

        loop {
            if Some(true)
                == family_parts
                    .peek()
                    .map(|n| n.chars().next().map(char::is_lowercase))
                    .flatten()
            {
                extra_given_parts.push(family_parts.next().unwrap())
            } else {
                break;
            }
        }

        write!(f, "{}", Join::new(" ", family_parts.map(Uppercase)))?;

        write!(f, ", {}", Initials(given))?;

        for e in extra_given_parts {
            write!(f, " {}", e)?;
        }

        Ok(())
    }
}

struct Initials<'i>(&'i str);

impl<'i> Display for Initials<'i> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut parts = self.0.split_whitespace();

        if let Some(s) = parts.next() {
            write!(f, "{}.", s.chars().next().unwrap())?;
        }

        for s in parts {
            // s will never be empty since many white spaces are considered
            // a single divisor.
            write!(f, " {}.", s.chars().next().unwrap())?;
        }

        Ok(())
    }
}

#[test]
fn test_surname_first() {
    assert_eq!(
        format!("{}", SurnameFirst("Araújo, G.")),
        "ARAÚJO, G.".to_string()
    );
    assert_eq!(
        format!("{}", SurnameFirst("del Priori, M.")),
        "PRIORI, M. del".to_string()
    );
}

#[test]
fn test_authors() {
    assert_eq!(
        format!("{}", Authors("Araújo, G.")),
        "ARAÚJO, G.".to_string()
    );

    assert_eq!(
        format!("{}", Authors("Araújo, G. AND Oliveira, F. I. D.")),
        "ARAÚJO, G.; OLIVEIRA, F. I. D.".to_string()
    );

    assert_eq!(
        format!(
            "{}",
            Authors("Araújo, G. AND Oliveira, F. I. D. AND de Tal, F. AND de Tal, S.")
        ),
        "ARAÚJO, G.; <em>et al</em>".to_string()
    );

    assert_eq!(format!("{}", Authors("Prado{ }Jr., C.")), "PRADO JR., C.",);
}

#[test]
fn test_initials() {
    assert_eq!(format!("{}", Authors("Araújo, Gabriel")), "ARAÚJO, G.",);
}

#[test]
fn single_name() {
    assert_eq!(format!("{}", Authors("Heródoto")), "HERÓDOTO",)
}
