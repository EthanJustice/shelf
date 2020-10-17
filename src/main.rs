// std

// external
use clap::{App, Arg, SubCommand};

// local
use shelf::Book;

fn main() {
    let app = App::new("shelf")
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .subcommand(SubCommand::with_name("view").about("View and query books."))
        .subcommand(
            SubCommand::with_name("add")
                .about("Add a new book")
                .arg(
                    Arg::with_name("title")
                        .short("t")
                        .help("The title of the book.")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("author")
                        .short("a")
                        .help("The book's author")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("genre")
                        .short("g")
                        .help("The book's primary genre")
                        .required(false)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("tags")
                        .help("Apply custom categories to the book")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("read")
                        .short("r")
                        .required(false)
                        .help("Specifies if the book has been read")
                        .takes_value(true)
                        .possible_values(&["true", "false"])
                        .default_value("false"),
                )
                .arg(
                    Arg::with_name("copies")
                        .short("c")
                        .help("Specifies the number of copies of the book.")
                        .takes_value(true)
                        .required(false)
                        .default_value("1"),
                ),
        )
        .subcommand(SubCommand::with_name("remove").about("Remove a book"))
        .get_matches();

    if let Some(_subcommand) = app.subcommand_matches("view") {
        // ...
    } else if let Some(_subcommand) = app.subcommand_matches("add") {
    } else if let Some(_subcommand) = app.subcommand_matches("remove") {
    } else {
    }
}
