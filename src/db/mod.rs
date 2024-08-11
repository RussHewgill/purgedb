pub mod history;

use std::collections::HashMap;

use crate::{
    gui::new_filament::NewFilament,
    types::{Filament, FilamentMap},
};
use hex_color::HexColor;
use rusqlite::{params, Connection, Result};

const CACHE_DURATION: std::time::Duration = std::time::Duration::from_secs(30);

pub struct Db {
    // db: Connection,
    pub db: Connection,

    cache_filaments: (FilamentMap, Vec<(u32, Filament)>),
    last_updated_filament: std::time::Instant,
    stale_filament: bool,

    cache_history: Vec<()>,
    last_updated_history: std::time::Instant,
    stale_history: bool,
}

impl Default for Db {
    fn default() -> Self {
        let db = Self::new().unwrap();
        // db.test_filaments().unwrap();
        db
    }
}

/// get filament
impl Db {
    pub fn is_stale_filament(&self) -> bool {
        self.stale_filament || self.last_updated_filament.elapsed() > CACHE_DURATION
    }

    pub fn get_filament(&self, id: u32) -> Result<Filament> {
        if !self.is_stale_filament() {
            if let Some(f) = self.cache_filaments.0.get(&id).cloned() {
                return Ok(f);
            }
        }

        eprintln!("get_filament");

        self.db.query_row(
            "SELECT 
        id, 
        name, 
        manufacturer, 
        color1, 
        color2, 
        color3, 
        material, 
        notes 
      FROM filaments
      WHERE id=?1
      ",
            [id],
            |row| {
                let id: u32 = row.get(0)?;
                let name: String = row.get(1)?;
                let manufacturer: String = row.get(2)?;
                let color: i32 = row.get(3)?;
                let color2: i32 = row.get(4)?;
                let color3: i32 = row.get(5)?;
                let material: String = row.get(6)?;
                let notes: String = row.get(7)?;

                let color = hex_color::HexColor::from_u24(color as u32);

                let mut colors = vec![];

                if color2 != -1 {
                    colors.push(hex_color::HexColor::from_u24(color2 as u32))
                }
                if color3 != -1 {
                    colors.push(hex_color::HexColor::from_u24(color3 as u32))
                }

                Ok(Filament::new(
                    id,
                    name,
                    manufacturer,
                    color,
                    &colors,
                    material,
                    notes,
                ))
            },
        )
    }

    pub fn get_all_filaments(&mut self) -> Result<(FilamentMap, Vec<(u32, Filament)>)> {
        if !self.is_stale_filament() {
            return Ok(self.cache_filaments.clone());
        }

        eprintln!("get_all_filaments");

        let mut stmt = self.db.prepare(
            "SELECT 
        id, 
        name, 
        manufacturer, 
        color1, 
        color2, 
        color3, 
        material, 
        notes 
      FROM filaments
      ORDER BY manufacturer COLLATE NOCASE ASC, name ASC
      ",
        )?;
        let iter = stmt.query_map([], |row| {
            let id: u32 = row.get(0)?;
            let name: String = row.get(1)?;
            let manufacturer: String = row.get(2)?;
            let color: i32 = row.get(3)?;
            let color2: i32 = row.get(4)?;
            let color3: i32 = row.get(5)?;
            let material: String = row.get(6)?;
            let notes: String = row.get(7)?;

            // let color = csscolorparser::parse(&color).unwrap();
            let color = hex_color::HexColor::from_u24(color as u32);

            let mut colors = vec![];

            if color2 != -1 {
                colors.push(hex_color::HexColor::from_u24(color2 as u32))
            }
            if color3 != -1 {
                colors.push(hex_color::HexColor::from_u24(color3 as u32))
            }

            Ok(Filament::new(
                id,
                name,
                manufacturer,
                color,
                &colors,
                material,
                notes,
            ))
        })?;
        let xs = iter
            .flatten()
            // .enumerate()
            // .map(|(i, x)| (i as u32, x))
            .map(|f| (f.id, f))
            .collect::<Vec<_>>();
        // let map = FilamentMap::new(xs.iter().map(|x| (x.id, x.clone())).collect());
        let map = FilamentMap::new(xs.iter().map(|(i, x)| (*i as u32, x.clone())).collect());

        self.cache_filaments = (map.clone(), xs.clone());
        self.stale_filament = false;
        self.last_updated_filament = std::time::Instant::now();

        Ok((map, xs))
    }

    #[cfg(feature = "nope")]
    pub fn get_all_searchable_keywords(&self) -> Result<crate::search::Keywords> {
        let names = self.get_all_names()?;
        let colors = self.get_all_colors()?;
        Ok(crate::search::Keywords::new(names, colors))
    }

    pub fn get_all_names(&self) -> Result<Vec<(u32, String)>> {
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

#[cfg(feature = "nope")]
impl Db {
    pub fn test_filaments(&self) -> Result<()> {
        // self.add_filament(&NewFilament::new("Polylite", "Polymaker", "#FFFFFF", "PLA"))?;
        // self.add_filament(&NewFilament::new("Polylite", "Polymaker", "#000000", "PLA"))?;
        // self.add_filament(&NewFilament::new("Polyterra", "Polymaker", "#5969cf", "PLA"))?;
        // self.add_filament(&NewFilament::new("Burnt Titanium", "Voxelab", "#121145", "PLA"))?;

        // self.add_filament(&NewFilament::new(
        //   "PolyLite",
        //   "Polymaker",
        //   [0xff, 0xff, 0xff],
        //   &[],
        // ))?;
        // self.add_filament(&NewFilament::new(
        //   "PolyLite",
        //   "Polymaker",
        //   [0x00, 0x00, 0x00],
        //   &[],
        // ))?;
        // self.add_filament(&NewFilament::new(
        //   "Candy Rainbow",
        //   "ERYONE",
        //   [0xec, 0x9b, 0xa4],
        //   &[[0xbb, 0xe3, 0x3d]],
        // ))?;
        // self.add_filament(&NewFilament::new(
        //   "Blue-Green-Orange",
        //   "ERYONE",
        //   [0x06, 0x9a, 0x2e],
        //   &[[0x2a, 0x60, 0x99], [0xff, 0x80, 0x00]],
        // ))?;

        Ok(())
    }
}

/// new, modify filament
impl Db {
    pub fn delete_filament(&mut self, id: u32) -> Result<()> {
        match self.db.execute(
            "DELETE FROM filaments WHERE id = ?1",
            [id],
            //
        ) {
            Ok(_) => (),
            Err(e) => eprintln!("e = {:?}", e),
            // Err(e) => {}
        }
        self.stale_filament = true;
        Ok(())
    }

    pub fn add_filament(&mut self, filament: &NewFilament, id: Option<u32>) -> Result<()> {
        // fn get_col(c: [u8; 3]) -> String {
        //   format!("#{:02X}{:02X}{:02X}", c[0], c[1], c[2])
        // }

        fn get_col(c: Option<[u8; 3]>) -> i32 {
            // format!("#{:02X}{:02X}{:02X}", c[0], c[1], c[2])
            match c {
                // Some(c) => 0i32 | c[0] as i32 | ((c[1] as i32) << 8)| ((c[2] as i32) << 16),
                Some(c) => {
                    let c = HexColor::rgb(c[0], c[1], c[2]);
                    c.to_u24() as i32
                }
                None => -1,
            }
        }

        let c1 = get_col(Some(filament.color_base.0));
        let c2 = get_col(filament.colors.get(0).map(|(c, _)| *c));
        let c3 = get_col(filament.colors.get(1).map(|(c, _)| *c));

        // eprintln!("c1 = {:?}", c1);
        // eprintln!("c2 = {:?}", c2);

        if let Some(id) = id {
            eprintln!("updating filament");
            match self.db.execute(
        "INSERT OR REPLACE INTO filaments (id, name, manufacturer, color1, color2, color3, material, notes) 
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        (
          id,
          &filament.name,
          &filament.manufacturer,
          c1,
          c2,
          c3,
          &filament.material,
          &filament.notes,
        ),
      ) {
        Ok(_) => (),
        Err(e) => eprintln!("e = {:?}", e),
        // Err(e) => {}
      }
        } else {
            match self.db.execute(
        "INSERT INTO filaments (name, manufacturer, color1, color2, color3, material, notes) 
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        (
          &filament.name,
          &filament.manufacturer,
          c1,
          c2,
          c3,
          &filament.material,
          &filament.notes,
        ),
      ) {
        Ok(_) => (),
        // Err(e) => eprintln!("e = {:?}", e),
        Err(e) => {}
      }
        }

        self.stale_filament = true;
        Ok(())
    }

    pub fn set_purge_values(&mut self, id_from: u32, id_to: u32, purge: u32) -> Result<()> {
        match self.db.execute(
            "INSERT OR REPLACE INTO purge_values (id_from, id_to, purge) VALUES (?1, ?2, ?3)",
            (id_from, id_to, purge),
        ) {
            Ok(_) => (),
            Err(e) => eprintln!("e = {:?}", e),
        }
        self.stale_filament = true;
        Ok(())
    }

    pub fn get_purge_values(&self, id_from: u32, id_to: u32) -> Result<u32> {
        self.db.query_row(
            "SELECT purge FROM purge_values WHERE id_from=?1 AND id_to=?2",
            (id_from, id_to),
            |row| row.get(0),
        )
    }

    pub fn new() -> Result<Self> {
        let path = "test.db";

        let conn = Connection::open(path)?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS filaments (
          id            INTEGER PRIMARY KEY,
          name          TEXT,
          manufacturer  TEXT NOT NULL,
          color1        INTEGER NOT NULL,
          color2        INTEGER NOT NULL,
          color3        INTEGER NOT NULL,
          material      TEXT,
          notes         TEXT,
          UNIQUE(name, manufacturer, color1, color2, color3)
      )",
            (), // empty list of parameters.
        )?;
        // material      TEXT NOT NULL,

        conn.execute(
            "CREATE TABLE IF NOT EXISTS purge_values (
          id          INTEGER PRIMARY KEY,
          id_from     INTEGER NOT NULL,
          id_to       INTEGER NOT NULL,
          purge       INTEGER NOT NULL,
          UNIQUE(id_from, id_to)
      )",
            (), // empty list of parameters.
        )?;

        Self::init_history(&conn)?;

        Ok(Self {
            db: conn,
            cache_filaments: (FilamentMap::new(HashMap::new()), vec![]),
            last_updated_filament: std::time::Instant::now(),
            stale_filament: true,

            cache_history: vec![],
            last_updated_history: std::time::Instant::now(),
            stale_history: true,
        })
    }
}
