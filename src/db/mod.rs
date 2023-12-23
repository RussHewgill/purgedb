use rusqlite::{params, Connection, Result};

pub struct Db {

}

impl Db {

  pub fn new() -> Self {
    Db {}
  }

  pub fn init(&self) -> Result<()> {

    let path = "test.db";

    let conn = Connection::open(path)?;

    conn.execute(
      "CREATE TABLE IF NOT EXISTS filaments (
          id            INTEGER PRIMARY KEY,
          name          TEXT NOT NULL,
          manufacturer  TEXT NOT NULL,
          color         TEXT NOT NULL,
          material      TEXT NOT NULL
      )",
      (), // empty list of parameters.
    )?;

    conn.execute(
      "CREATE TABLE IF NOT EXISTS purge values (
          id          INTEGER PRIMARY KEY,
          id_from     INTEGER NOT NULL,
          id_to       INTEGER NOT NULL,
          purge_from  INTEGER NOT NULL,
          purge_to    INTEGER NOT NULL
      )",
      (), // empty list of parameters.
    )?;

    Ok(())
  }
}
