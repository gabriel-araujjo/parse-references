use std::borrow::Borrow;
use 

#[derive(FromTag)]
struct Book {
    title: String,
    author: String,
    year: String,
    location: String,
    publisher: String,
}

#[test]
fn derive() {
    let tags = [
        ("title".to_string(), "title".to_string()),
        ("author".to_string(), "author".to_string()),
        ("year".to_string(), "year".to_string()),
        ("location".to_string(), "location".to_string()),
        ("publisher".to_string(), "publisher".to_string()),
    ];


}
