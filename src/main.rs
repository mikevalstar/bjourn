#![allow(dead_code)]
#![allow(unused_imports)]
#[path = "lib/bargs.rs"]
mod bargs;
#[path = "lib/db.rs"]
mod db;

use chrono;
use colored::Colorize;
use exitcode;

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
    let args: Vec<String> = std::env::args().collect();
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
    if args.len() == 1 {
        // print out some usage info before the list
        println!("Usage:");
        println!(
            "\t{} {}",
            "bjourn".bold(),
            "[action] [args]".bold().italic()
        );
        println!("");
        println!(
            "\t{} {}",
            "bjourn list".bold(),
            "[optional date]".bold().italic()
        );
        println!("\t{}", "bjourn add my entry here".bold());
        println!("\t{}", "bjourn remove ZScG1V3i".bold());
        println!("");
        println!("Actions: {}", "add, list, remove".bold().italic());
        /*println!(
            "\t{} {}",
            "bjourn help".bold(),
            "- prints this help".italic()
        );*/
        println!("");

        let today = &chrono::Local::now().format("%Y-%m-%d").to_string();
        println!("Your journal for today: {}", today.bold());
        println!("");

        let list = db::list_bullets(today);
        if let Err(e) = list {
            println!("Error listing bullets: {}", e);
            std::process::exit(1);
        }
        for bullet in list.unwrap() {
            println!(
                "{} {}: {}",
                "*".bold(),
                bullet.quickid.magenta(),
                bullet.text
            );
        }

        std::process::exit(exitcode::OK);
    }

    // check if the first arg is a valid action or we default to "add"
    let action = if GLOBAL_ACTIONS.contains(&&args[1][..]) {
        &args[1]
    } else {
        "add"
    };

    // if "add" then take everything after the first arg and add it to a single string
    if action == "add" {
        let mut new_bullet = String::new();
        for i in 1..args.len() {
            if i == 1 && args[i] == "add" {
                continue;
            }
            new_bullet.push_str(&args[i]);
            new_bullet.push_str(" ");
        }
        if env_debug {
            println!("Adding: {}", new_bullet);
        }

        if let Err(e) = db::add_bullet(&new_bullet) {
            eprintln!("Error adding bullet: {}", e);
            std::process::exit(exitcode::IOERR);
        }
    }

    // handle the "list" action
    if action == "list" {
        // read in the date as the second arg (if blank use today)
        let date = if args.len() > 2 {
            &args[2]
        } else {
            &chrono::Local::now().format("%Y-%m-%d").to_string()
        };

        let list = db::list_bullets(date);
        if let Err(e) = list {
            eprintln!("Error listing bullets: {}", e);
            std::process::exit(exitcode::IOERR);
        }
        for bullet in list.unwrap() {
            println!(
                "{} {}: {}",
                "*".bold(),
                bullet.quickid.magenta(),
                bullet.text
            );
        }
    }

    // remove
    if action == "remove" {
        if args.len() < 3 {
            eprintln!("Error: remove requires a quickid");
            std::process::exit(exitcode::USAGE);
        }
        let quickid = &args[2];
        if env_debug {
            println!("Removing: {}", quickid);
        }

        if let Err(e) = db::remove_bullet(quickid) {
            eprintln!("Error removing bullet: {}", e);
            std::process::exit(exitcode::IOERR);
        }
    }

    std::process::exit(exitcode::OK);
}
