use clap::{Arg, Command};

mod core;

fn main() {
    // TODO: Move to separate CLI bin
    let m = Command::new("hui")
        .subcommand(
            Command::new("collection")
                .subcommand(
                    Command::new("add")
                        .args([Arg::new("path").default_value("")])
                        .about("Adds a new collection to provided path"),
                )
                .subcommand(Command::new("delete").about("Deletes a collection"))
                .subcommand(Command::new("ls").about("Prints Collection tree"))
                .about("Work on collections"),
        )
        // TODO: 'send'
        .subcommand(Command::new("send").about("Send requests"))
        .get_matches();

    // Handlers here
    match m.subcommand() {
        _ => {}
    }
}
