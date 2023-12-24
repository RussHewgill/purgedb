use rusqlite::{params, Connection, Result};
use crate::{gui::new_filament::NewFilament, search::Keywords, types::Filament};

pub struct Db {
  db: Connection,
}

impl Db {
  pub fn get_all_filaments(&self) -> Result<Vec<Filament>> {
    let mut stmt = self.db.prepare("SELECT id, name, manufacturer, color1 FROM filaments")?;
    let iter = stmt.query_map([], |row| {
      let id: u32 = row.get(0)?;
      let name: String = row.get(1)?;
      let manufacturer: String = row.get(2)?;
      let color: String = row.get(3)?;
      // let material: String = row.get(4)?;

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
    let mut stmt = self.db.prepare("SELECT id, name FROM filaments")?;
    let names_iter = stmt.query_map([], |row| {
      let id: u32 = row.get(0)?;
      let name: String = row.get(1)?;
      Ok((id, name))
    })?;
    Ok(names_iter.flatten().collect())
  }

  fn get_all_colors(&self) -> Result<Vec<(u32, String)>> {
    let mut stmt = self.db.prepare("SELECT id, color FROM filaments")?;
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

    let c1 = format!("#{:02X}{:02X}{:02X}", filament.color1[0],filament.color1[1],filament.color1[2]);
    // let c2 = format!("#{:02X}{:02X}{:02X}", filament.color2[0],filament.color2[1],filament.color2[2]);
    // let c3 = format!("#{:02X}{:02X}{:02X}", filament.color3[0],filament.color3[1],filament.color3[2]);

    match self.db.execute(
      "INSERT INTO filaments (name, manufacturer, color1) VALUES (?1, ?2, ?3)", 
      (&filament.name, &filament.manufacturer, c1)
      // "INSERT INTO filaments (name, manufacturer, c1, c2, c3) VALUES (?1, ?2, ?3, ?4, ?5)", 
      // (&filament.name, &filament.manufacturer, c1, c2, c3)
    ) {
        Ok(_) => (),
        // Err(e) => eprintln!("e = {:?}", e),
        Err(e) => {},
    }

    Ok(())
  }

  pub fn set_purge_values(&self, id_from: u32, id_to: u32, purge: u32) -> Result<()> {
    match self.db.execute(
      "INSERT OR REPLACE INTO purge_values (id_from, id_to, purge) VALUES (?1, ?2, ?3)", 
      (id_from, id_to, purge)
    ) {
        Ok(_) => (),
        Err(e) => eprintln!("e = {:?}", e),
    }
    Ok(())
  }

  pub fn get_purge_values(&self, id_from: u32, id_to: u32) -> Result<u32> {
    self.db.query_row(
      "SELECT purge FROM purge_values WHERE id_from=?1 AND id_to=?2", 
      (id_from, id_to),
      |row| {
        row.get(0)
      }
    )
  }

  pub fn test_filaments(&self) -> Result<()> {
    // self.add_filament(&NewFilament::new("Polylite", "Polymaker", "#FFFFFF", "PLA"))?;
    // self.add_filament(&NewFilament::new("Polylite", "Polymaker", "#000000", "PLA"))?;
    // self.add_filament(&NewFilament::new("Polyterra", "Polymaker", "#5969cf", "PLA"))?;
    // self.add_filament(&NewFilament::new("Burnt Titanium", "Voxelab", "#121145", "PLA"))?;

    self.add_filament(&NewFilament::new("PolyLite", "Polymaker", [0xff, 0xff, 0xff]))?;
    self.add_filament(&NewFilament::new("PolyLite", "Polymaker", [0x00, 0x00, 0x00]))?;

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
          color1        TEXT NOT NULL,
          color2        TEXT,
          color3        TEXT,
          UNIQUE(name, manufacturer, color1)
      )",
      (), // empty list of parameters.
    )?;
    // material      TEXT NOT NULL,

    conn.execute(
      "CREATE TABLE IF NOT EXISTS purge_values (
          id_from     INTEGER PRIMARY KEY,
          id_to       INTEGER NOT NULL,
          purge       INTEGER NOT NULL,
          UNIQUE(id_from, id_to)
      )",
      (), // empty list of parameters.
    )?;

    Ok(Self {
      db: conn,
    })
  }
}
