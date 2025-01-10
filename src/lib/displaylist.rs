use crate::bargs;
use crate::bargs::BJournRunner;
use crate::db;
use colored::Colorize;
use serde_json::json;
use std::io::IsTerminal;

fn format_line(format: &str, bullet: db::BItem) -> String {
    let mut content = format.to_string();

    let bdate = chrono::NaiveDate::parse_from_str(&bullet.list_date, "%Y-%m-%d").unwrap();
    let added_date =
        chrono::NaiveDateTime::parse_from_str(&bullet.added, "%Y-%m-%d %H:%M:%S").unwrap();

    // replace for all the values
    content = content.replace("{quickid}", &bullet.quickid);
    content = content.replace("{bullet}", &bullet.text);
    content = content.replace("{date}", &bdate.format("%Y-%m-%d").to_string());
    content = content.replace(
        "{{added}}",
        &added_date.format("%Y-%m-%d %H:%M:%S").to_string(),
    );
    content = content.replace("{yyyy}", &added_date.format("%Y").to_string());
    content = content.replace("{mm}", &added_date.format("%m").to_string());
    content = content.replace("{dd}", &added_date.format("%d").to_string());
    content = content.replace("{HH}", &added_date.format("%H").to_string());
    content = content.replace("{MM}", &added_date.format("%M").to_string());
    content = content.replace("{SS}", &added_date.format("%S").to_string());

    content
}

fn displaylist_md_row_terminal(itm: db::BItem, format: &String) {
    if format == "{default}" {
        println!("{} {}: {}", "*".bold(), itm.quickid.magenta(), itm.text);
    } else {
        println!("{}", format_line(format, itm));
    }
}

fn displaylist_md_row(itm: db::BItem, format: &String) {
    if format == "{default}" {
        println!("* {}", itm.text);
    } else {
        println!("{}", format_line(format, itm));
    }
}

fn displaylist_md(list: Vec<db::BItem>, format: String) {
    for bullet in list {
        if std::io::stdout().is_terminal() {
            displaylist_md_row_terminal(bullet, &format);
        } else {
            // for piping output
            displaylist_md_row(bullet, &format);
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

    let line_format: String = match args.flag_arg("format") {
        Some(val) => val,
        None => "{default}".to_string(),
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
        "md" => displaylist_md(list.unwrap(), line_format),
        "markdown" => displaylist_md(list.unwrap(), line_format),
        "json" => displaylist_json(list.unwrap()),
        _ => eprintln!("Unknown format: {}", format),
    }
}
