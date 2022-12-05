use std::fmt::Display;

use crate::abnt::join::{Join, JoinAnd};

use super::{AND_REGEX, tex};

pub struct LocationPublisher<'a>(pub &'a str, pub &'a str);

struct SingleLocPub<'a>(pub &'a str, pub &'a str);

impl<'a> Display for SingleLocPub<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (self.0.is_empty(), self.1.is_empty()) {
            (true, true) => write!(f, "[s.l.: s.n.]"),
            (true, false) => write!(f, "[s.l.]: {}", tex::Text(self.1)),
            (false, true) => write!(f, "{}: [s.n.]", tex::Text(self.0)),
            (false, false) => write!(f, "{}: {}", tex::Text(self.0), tex::Text(self.1)),
        }
    }
}

impl<'a> Display for LocationPublisher<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let locations: Vec<_> = AND_REGEX.split(self.0).collect();

        let publishers: Vec<_> = AND_REGEX.split(self.1).collect();

        if locations.len() == publishers.len() {
            let loc_pub = locations
                .into_iter()
                .zip(publishers)
                .map(|(loc, publ)| SingleLocPub(loc, publ));

            write!(f, "{}", Join::new("; ", loc_pub))
        } else {
            write!(
                f,
                "{}: {}",
                JoinAnd::new(", ", " e ", locations.into_iter()),
                JoinAnd::new(", ", " e ", publishers.into_iter()),
            )
        }
    }
}
