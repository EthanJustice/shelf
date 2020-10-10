// std

// external
use clap::{App, Arg, SubCommand};

// local

fn main() {
    let app = App::new("shelf")
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .subcommand(SubCommand::with_name("view").help_short("View and query books."))
        .subcommand(SubCommand::with_name("add").help_short("Add a new book"))
        .subcommand(SubCommand::with_name("remove").help_short("Remove a book"))
        .get_matches();

    if let Some(_subcommand) = app.subcommand_matches("view") {
        // ...
    }
}
