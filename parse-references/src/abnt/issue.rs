use std::fmt::Display;


#[repr(transparent)]
pub struct Issue<'i>(pub &'i str);

impl<'i> Display for Issue<'i> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "n. {}", self.0)
    }
}

#[test]
fn issue() {
    let iss = Issue("5");

    assert_eq!(format!("{iss}"), "n. 5");
}
