use homedir::my_home;
use rusqlite::{params, Connection, Result};

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
  }else {
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