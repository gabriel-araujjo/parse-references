use std::fmt::Display;


#[repr(transparent)]
pub struct Volume<'v>(pub &'v str);

impl<'v> Display for Volume<'v> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "v. {}", self.0)
    }
}

#[test]
fn volume() {
    let vol = Volume("5");

    assert_eq!(format!("{vol}"), "v. 5");
}
