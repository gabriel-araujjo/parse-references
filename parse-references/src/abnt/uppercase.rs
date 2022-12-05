use std::fmt::Display;

pub struct Uppercase<'u>(pub &'u str);

impl<'u> Display for Uppercase<'u> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in self.0.chars() {
            if c == '{' || c == '}' {
                continue;
            }
            write!(f, "{}", c.to_uppercase())?;
        }
        Ok(())
    }
}

#[test]
fn remove_braces() {
    let s = "Prado{ }Jr.";

    assert_eq!(
        format!("{}", Uppercase(s)),
        "PRADO JR.",
    )
}
