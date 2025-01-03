#![allow(dead_code)]
#![allow(unused_imports)]
#[path = "lib/bargs.rs"]
mod bargs;

#[path = "lib/db.rs"]
mod db;

use colored::Colorize;
use std::io::IsTerminal;

// a list of first arg options enum
static GLOBAL_ACTIONS: [&str; 3] = ["add", "list", "remove"];

fn main() {
    let mut env_debug = false;

    if std::env::var("DEBUG").is_ok() {
        if let Ok(val) = std::env::var("DEBUG") {
            if val == "true" {
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

    // if 0 args, print help
    if args.action == bargs::BAction::ListDefault {
        let today = &chrono::Local::now().format("%Y-%m-%d").to_string();

        if std::io::stdout().is_terminal() {
            // print out some usage info before the list, but only if it's a terminal
            println!("Usage:");
            println!(
                "\t{} {}",
                "bjourn".bold(),
                "[action] [args]".bold().italic()
            );
            println!();
            println!(
                "\t{} {}",
                "bjourn list".bold(),
                "[optional date]".bold().italic()
            );
            println!("\t{}", "bjourn add my entry here".bold());
            println!("\t{}", "bjourn remove ZScG1V3i".bold());
            println!();
            println!("Actions: {}", "add, list, remove".bold().italic());
            /*println!(
                "\t{} {}",
                "bjourn help".bold(),
                "- prints this help".italic()
            );*/
            println!();

            println!("Your journal for today: {}", today.bold());
            println!();
        }

        let list = db::list_bullets(today);
        if let Err(e) = list {
            println!("Error listing bullets: {}", e);
            std::process::exit(1);
        }
        for bullet in list.unwrap() {
            if std::io::stdout().is_terminal() {
                println!(
                    "{} {}: {}",
                    "*".bold(),
                    bullet.quickid.magenta(),
                    bullet.text
                );
            } else {
                // for piping output
                println!("* {}", bullet.text);
            }
        }

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

    // handle the list action
    if args.action == bargs::BAction::List {
        // read in the date as the second arg (if blank use today)
        let date = match args.input {
            Some(ref d) => d,
            None => &chrono::Local::now().format("%Y-%m-%d").to_string(),
        };

        let list = db::list_bullets(date);
        if let Err(e) = list {
            eprintln!("Error listing bullets: {}", e);
            std::process::exit(exitcode::IOERR);
        }
        for bullet in list.unwrap() {
            if std::io::stdout().is_terminal() {
                println!(
                    "{} {}: {}",
                    "*".bold(),
                    bullet.quickid.magenta(),
                    bullet.text
                );
            } else {
                // for piping output
                println!("* {}", bullet.text);
            }
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

    std::process::exit(exitcode::OK);
}
