use std::fmt::Display;

pub struct Strong<T>(pub T);

impl<T: Display> Display for Strong<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<strong>{}</strong>", self.0)
    }
}
