use rusqlite::{params, Connection, Result};
use crate::{gui::new_filament::NewFilament, search::Keywords, types::Filament};

pub struct Db {
  conn: Connection,
}

impl Db {
  pub fn get_all_filaments(&self) -> Result<Vec<Filament>> {
    let mut stmt = self.conn.prepare("SELECT id, name, manufacturer, color, material FROM filaments")?;
    let iter = stmt.query_map([], |row| {
      let id: u32 = row.get(0)?;
      let name: String = row.get(1)?;
      let manufacturer: String = row.get(2)?;
      let color: String = row.get(3)?;
      let material: String = row.get(4)?;

      // let color = csscolorparser::parse(&color).unwrap();
      let color = hex_color::HexColor::parse(&color).unwrap();

      Ok(Filament::new(id, name, manufacturer, color))
    })?;
    Ok(iter.flatten().collect())
  }

  pub fn get_all_searchable_keywords(&self) -> Result<Keywords> {

    let names = self.get_all_names()?;
    let colors = self.get_all_colors()?;

    Ok(Keywords::new(
      names,
      colors,
    ))
  }

  fn get_all_names(&self) -> Result<Vec<(u32, String)>> {
    let mut stmt = self.conn.prepare("SELECT id, name FROM filaments")?;
    let names_iter = stmt.query_map([], |row| {
      let id: u32 = row.get(0)?;
      let name: String = row.get(1)?;
      Ok((id, name))
    })?;
    Ok(names_iter.flatten().collect())
  }

  fn get_all_colors(&self) -> Result<Vec<(u32, String)>> {
    let mut stmt = self.conn.prepare("SELECT id, color FROM filaments")?;
    let colors_iter = stmt.query_map([], |row| {
      let id: u32 = row.get(0)?;
      let color: String = row.get(3)?;
      Ok((id, color))
    })?;
    Ok(colors_iter.flatten().collect())
  }

}

impl Db {
  pub fn add_filament(&self, filament: &NewFilament) -> Result<()> {

    match self.conn.execute(
      "INSERT INTO filaments (name, manufacturer, color, material) VALUES (?1, ?2, ?3, ?4)", 
      (&filament.name, &filament.manufacturer, &filament.color, &filament.material)
    ) {
        Ok(_) => (),
        Err(e) => eprintln!("e = {:?}", e),
    }

    Ok(())
  }

  pub fn test_filaments(&self) -> Result<()> {
    self.add_filament(&NewFilament::new("Polylite", "Polymaker", "#FFFFFF", "PLA"))?;
    self.add_filament(&NewFilament::new("Polylite", "Polymaker", "#000000", "PLA"))?;
    self.add_filament(&NewFilament::new("Polyterra", "Polymaker", "#5969cf", "PLA"))?;
    self.add_filament(&NewFilament::new("Burnt Titanium", "Voxelab", "#121145", "PLA"))?;

    Ok(())
  }

  pub fn new() -> Result<Self> {

    let path = "test.db";

    let conn = Connection::open(path)?;

    conn.execute(
      "CREATE TABLE IF NOT EXISTS filaments (
          id            INTEGER PRIMARY KEY,
          name          TEXT,
          manufacturer  TEXT NOT NULL,
          color         TEXT NOT NULL,
          material      TEXT NOT NULL,
          UNIQUE(name, manufacturer, color, material)
      )",
      (), // empty list of parameters.
    )?;

    conn.execute(
      "CREATE TABLE IF NOT EXISTS purge_values (
          id          INTEGER PRIMARY KEY,
          id_from     INTEGER NOT NULL,
          id_to       INTEGER NOT NULL,
          purge_from  INTEGER NOT NULL,
          purge_to    INTEGER NOT NULL,
          UNIQUE(id_from, id_to)
      )",
      (), // empty list of parameters.
    )?;

    Ok(Self {
      conn,
    })
  }
}
