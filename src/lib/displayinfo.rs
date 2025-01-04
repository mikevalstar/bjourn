use colored::Colorize;

pub fn usage() {
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
    println!(
        "\t{} {}",
        "bjourn help".bold(),
        "- prints this help".italic()
    );
    println!();
}

pub fn version() {
    let version = env!("CARGO_PKG_VERSION");
    println!("bjourn {}", version);
}

pub fn help() {
    let version = env!("CARGO_PKG_VERSION");
    println!("{} {}", "bjourn".green(), version);
    println!();
    println!("A simple journaling tool");
    println!();
    println!("{}", "USAGE:".yellow());
    println!("\tbjourn [action] [args]");
    println!();

    println!("{}", "ACTIONS:".yellow());

    println!("{}", "\t-a, --add, add [text]".green());
    println!("\t\tAdd a new entry with the given text");

    println!("{}", "\t-h, --help, help".green());
    println!("\t\tPrint this help message");

    println!("{}", "\t-l, --list, list [optional date]".green());
    println!("\t\tList all entries for the given date, defaults to today");

    println!("{}", "\t-r, --remove, remove [id]".green());
    println!("\t\tRemove the entry with the given id");

    println!("{}", "\t-v, --version, version".green());
    println!("\t\tPrint the version of bjourn");

    println!();

    println!("{}", "OPTIONS:".yellow());
    println!("{}", "\t-o, --output [md, markdown, json]".green());
}
