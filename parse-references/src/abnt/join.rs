use std::{cell::Cell, fmt::Display, mem::swap};

pub struct Join<I> {
    sep: &'static str,
    items: Cell<Option<I>>,
}

impl<I> Join<I> {
    pub fn new(sep: &'static str, items: I) -> Self {
        Self {
            sep,
            items: Cell::new(Some(items)),
        }
    }
}

impl<I> Display for Join<I>
where
    I: Iterator,
    <I as Iterator>::Item: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.items.replace(None).ok_or(std::fmt::Error)?;

        if let Some(item) = iter.next() {
            write!(f, "{item}")?;
        } else {
            return Ok(());
        }

        for item in iter {
            f.write_str(self.sep)?;
            write!(f, "{item}")?;
        }

        Ok(())
    }
}

pub struct JoinAnd<I> {
    sep: &'static str,
    and: &'static str,
    items: Cell<Option<I>>,
}

impl<I> JoinAnd<I> {
    pub fn new(sep: &'static str, and: &'static str, items: I) -> Self {
        Self {
            sep,
            and,
            items: Cell::new(Some(items)),
        }
    }
}

impl<I> Display for JoinAnd<I>
where
    I: Iterator,
    <I as Iterator>::Item: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.items.replace(None).ok_or(std::fmt::Error)?;

        if let Some(item) = iter.next() {
            write!(f, "{item}")?;
        } else {
            return Ok(());
        };

        let mut latest = if let Some(item) = iter.next() {
            item
        } else {
            return Ok(());
        };

        for mut item in iter {
            swap(&mut item, &mut latest);
            write!(f, "{}{item}", self.sep)?;
        }

        write!(f, "{}{latest}", self.and)
    }
}
