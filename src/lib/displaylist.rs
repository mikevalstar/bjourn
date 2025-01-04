use crate::bargs;
use crate::bargs::BJournRunner;
use crate::db;
use colored::Colorize;
use serde_json::json;
use std::io::IsTerminal;

fn displaylist_md_row_terminal(itm: db::BItem) {
    println!("{} {}: {}", "*".bold(), itm.quickid.magenta(), itm.text);
}

fn displaylist_md_row(itm: db::BItem) {
    println!("* {}", itm.text);
}

fn displaylist_md(list: Vec<db::BItem>) {
    for bullet in list {
        if std::io::stdout().is_terminal() {
            displaylist_md_row_terminal(bullet);
        } else {
            // for piping output
            displaylist_md_row(bullet);
        }
    }
}

fn displaylist_json(list: Vec<db::BItem>) {
    let mut items = Vec::new();
    for bullet in list {
        items.push(json!({
            "quickid": bullet.quickid,
            "bullet": bullet.text,
            "date": bullet.list_date,
            "added": bullet.added,
        }));
    }

    println!("{}", json!(items));
}

pub fn displaylist(args: &bargs::BArgs) {
    let format = match args.flag_arg("output") {
        Some(val) => val,
        None => "md".to_string(),
    };

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

    match format.as_str() {
        "md" => displaylist_md(list.unwrap()),
        "markdown" => displaylist_md(list.unwrap()),
        "json" => displaylist_json(list.unwrap()),
        _ => eprintln!("Unknown format: {}", format),
    }
}
