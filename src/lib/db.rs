use chrono::prelude;
use homedir::my_home;
use nanoid::nanoid;
use rusqlite::{params, Connection, Result};

static ALPHABET: [char; 62] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9',
];

#[derive(Debug)]
pub struct BItem {
    pub id: i32,
    pub quickid: String,
    pub added: String,
    pub list_date: String,
    pub text: String,
}

// Lists the bullets for a given day
pub fn list_bullets(date: &str) -> Result<Vec<BItem>> {
    let db_path = database_location();
    let conn = Connection::open(db_path)?;

    let mut stmt =
        conn.prepare("SELECT id, quickid, text, list_date, added FROM blist WHERE list_date = ?1")?;
    let bullet_iter = stmt.query_map(params![date], |row| {
        Ok(BItem {
            id: row.get(0)?,
            quickid: row.get(1)?,
            added: row.get(4)?,
            list_date: row.get(3)?,
            text: row.get(2)?,
        })
    })?;

    let mut bullets = Vec::new();
    for bullet in bullet_iter {
        bullets.push(bullet?);
    }

    Ok(bullets)
}

// Adds a bullt with a random nano id
pub fn add_bullet(text: &String) -> Result<()> {
    let db_path = database_location();
    let conn = Connection::open(db_path)?;

    let now = chrono::Local::now();
    let now_str = now.format("%Y-%m-%d %H:%M:%S").to_string();
    let date_str = now.format("%Y-%m-%d").to_string();
    let quickid = nanoid!(8, &ALPHABET);

    conn.execute(
        "INSERT INTO blist (quickid, added, list_date, text) VALUES (?1, ?2, ?3, ?4)",
        params![quickid, now_str, date_str, text],
    )?;

    Ok(())
}

// Creates and sets up the DB schema
pub fn create_database() -> Result<()> {
    let db_path = database_location();

    // Connect to SQLite database (creates the file if it doesn't exist)
    let conn = Connection::open(db_path)?;

    // Create an env table for settings
    conn.execute(
        "CREATE TABLE IF NOT EXISTS env (
          id INTEGER PRIMARY KEY AUTOINCREMENT,
          key TEXT NOT NULL,
          value TEXT NOT NULL
      )",
        [], // No parameters needed
    )?;

    // Insert the current version if no key exists
    conn.execute(
        "INSERT OR IGNORE INTO env (key, value) VALUES ('version', '0.1.0')",
        [], // No parameters needed
    )?;

    // Create the bullet list
    conn.execute(
        "CREATE TABLE IF NOT EXISTS blist (
          id INTEGER PRIMARY KEY AUTOINCREMENT,
          quickid TEXT NOT NULL,
          added TEXT NOT NULL,
          list_date TEXT NOT NULL,
          text TEXT NOT NULL
      )",
        [], // No parameters needed
    )?;

    Ok(())
}

pub fn database_location() -> String {
    let db_path: String;

    if std::env::var("BLIST_DB").is_ok() {
        db_path = std::env::var("BLIST_DB").unwrap();
    } else {
        let db_dir = my_home().unwrap();
        match db_dir {
            Some(mut dir) => {
                dir.push(".blist.db");
                db_path = dir.into_os_string().into_string().unwrap();
            }
            None => {
                println!("Could not find home directory or BLIST_DB environment variable");
                std::process::exit(1);
            }
        }
    }

    return db_path;
}
