// std

// crates
use serde::Serialize;

// local

#[derive(Serialize)]
pub struct NoData();

#[derive(Serialize)]
pub struct Context<T: Serialize, S: Into<String>> {
    pub title: S,
    pub parent: &'static str,
    pub data: std::option::Option<T>,
}

#[derive(Serialize)]
pub struct Book {
    /// The title of the book
    pub title: &'static str,
    /// The book's primary genre
    pub genre: &'static str,
    /// The author of the book
    pub author: &'static str,
    /// Custom categories for the book
    pub tags: Vec<&'static str>,
    /// If the book has been read, the value will be `true`; otherwise, it will be `false`
    pub read: bool,
    /// Number of copies of the book
    pub copies: u8,
}
