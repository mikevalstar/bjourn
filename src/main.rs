#![allow(dead_code)]
#![allow(unused_imports)]
#[path = "lib/bargs.rs"]
mod bargs;

#[path = "lib/db.rs"]
mod db;

#[path = "lib/displayinfo.rs"]
mod displayinfo;

#[path = "lib/displaylist.rs"]
mod displaylist;

use colored::Colorize;
use std::io::IsTerminal;

// a list of first arg options enum
static GLOBAL_ACTIONS: [&str; 3] = ["add", "list", "remove"];

fn main() {
    let mut env_debug = false;
    let mut supress_usage = false;

    if std::env::var("BJOURN_USAGE").is_ok() {
        let usage = std::env::var("BJOURN_USAGE").unwrap();
        if usage == "false" || usage == "0" {
            supress_usage = true;
        }
    }

    if std::env::var("DEBUG").is_ok() {
        if let Ok(val) = std::env::var("DEBUG") {
            if val == "true" || val == "1" {
                println!("Debug mode is enabled");
                env_debug = true;
            }
        }
    }

    // read in the arguments
    let args = bargs::parse_args();
    if env_debug {
        dbg!(&args);
    }

    // Create the database if needed before exercising the actions
    let dbgo = db::create_database();
    if let Err(e) = dbgo {
        eprintln!("Error creating database: {}", e);
        std::process::exit(exitcode::CANTCREAT);
    }

    // version
    // TODO: move as a modifier so we can keep going
    if args.action == bargs::BAction::Version {
        displayinfo::version();
        std::process::exit(exitcode::OK);
    }

    // if 0 args, print totday with a breif usage details
    if args.action == bargs::BAction::ListDefault {
        let today = &chrono::Local::now().format("%Y-%m-%d").to_string();

        if std::io::stdout().is_terminal() {
            // print out some usage info before the list, but only if it's a terminal
            if !supress_usage {
                displayinfo::usage();
            }

            println!("Your journal for today: {}", today.bold());
            println!();
        }

        displaylist::displaylist(&args);

        std::process::exit(exitcode::OK);
    }

    // Help
    if args.action == bargs::BAction::Help {
        displayinfo::help();
        std::process::exit(exitcode::OK);
    }

    // if "add" then take everything after the first arg and add it to a single string
    if args.action == bargs::BAction::Add {
        let input = match &args.input {
            Some(t) => t,
            None => {
                eprintln!("Error: adding requires a text argument");
                std::process::exit(exitcode::USAGE);
            }
        };

        if env_debug {
            println!("Adding: {}", input);
        }

        if let Err(e) = db::add_bullet(input) {
            eprintln!("Error adding bullet: {}", e);
            std::process::exit(exitcode::IOERR);
        }
    }

    // remove
    if args.action == bargs::BAction::Remove {
        let input = match &args.input {
            Some(t) => t,
            None => {
                eprintln!("Error: remove requires a quickid");
                std::process::exit(exitcode::USAGE);
            }
        };

        if env_debug {
            println!("Removing: {}", input);
        }

        if let Err(e) = db::remove_bullet(input) {
            eprintln!("Error removing bullet: {}", e);
            std::process::exit(exitcode::IOERR);
        }
    }

    // handle the list action
    if args.action == bargs::BAction::List {
        displaylist::displaylist(&args);
    }

    std::process::exit(exitcode::OK);
}
