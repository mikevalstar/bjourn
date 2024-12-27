#[path = "lib/db.rs"] mod db;

// a list of first arg options enum
static GLOBAL_ACTIONS: [&str; 3] = [
    "add",
    "list",
    "remove"
];

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

    // if 0 args, print help
    if args.len() == 1 {
        // TODO just print out the day's list of bullet items
        println!("Say something!");
        std::process::exit(1);
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
        for i in 2..args.len() {
            new_bullet.push_str(&args[i]);
            new_bullet.push_str(" ");
        }
        if env_debug {
            println!("Adding: {}", new_bullet);
        }
    }

    // Create the database if needed 
    let dbgo = db::create_database();
    if let Err(e) = dbgo {
        println!("Error creating database: {}", e);
        std::process::exit(1);
    }

}

