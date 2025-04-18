pub mod history;

use std::collections::HashMap;

use crate::{
    gui::{MinSavedState, history_tab::HistoryRow, new_filament::NewFilament},
    types::{Filament, FilamentMap},
};
use hex_color::HexColor;
use rusqlite::{Connection, Result, params};

const CACHE_DURATION: std::time::Duration = std::time::Duration::from_secs(30);

pub struct Db {
    // db: Connection,
    db: Option<Connection>,

    cache_filaments: (FilamentMap, Vec<(u32, Filament)>),
    last_updated_filament: std::time::Instant,
    stale_filament: bool,

    cache_history: Vec<HistoryRow>,
    cache_history_sort: Option<(usize, crate::gui::history_tab::SortOrder)>,
    last_updated_history: std::time::Instant,
    stale_history: bool,
}

/// This is needed for serde, but shouldn't be actually called?
impl Default for Db {
    fn default() -> Self {
        let db_path = std::path::PathBuf::from("test.db");
        let db = Self::new(db_path).unwrap();
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

        self.db.as_ref().unwrap().query_row(
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
                    false,
                    false,
                ))
            },
        )
    }

    pub fn get_all_filaments(&mut self) -> Result<(FilamentMap, Vec<(u32, Filament)>)> {
        if !self.is_stale_filament() {
            return Ok(self.cache_filaments.clone());
        }

        eprintln!("get_all_filaments");

        let mut stmt = self.db.as_ref().unwrap().prepare(
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
                false,
                false,
            ))
        })?;
        let xs = iter.flatten().map(|f| (f.id, f)).collect::<Vec<_>>();

        let mut map = FilamentMap::new(xs.iter().map(|(i, x)| (*i as u32, x.clone())).collect());

        let (default_w, default_b) = self.get_default_black_white()?;
        if let Some(id) = default_w {
            if let Some(f) = map.filaments.get_mut(&id) {
                f.set_default_white();
            }
        }
        if let Some(id) = default_b {
            if let Some(f) = map.filaments.get_mut(&id) {
                f.set_default_black();
            }
        }

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
        let mut stmt = self
            .db
            .as_ref()
            .unwrap()
            .prepare("SELECT id, name FROM filaments")?;
        let names_iter = stmt.query_map([], |row| {
            let id: u32 = row.get(0)?;
            let name: String = row.get(1)?;
            Ok((id, name))
        })?;
        Ok(names_iter.flatten().collect())
    }

    fn get_all_colors(&self) -> Result<Vec<(u32, String)>> {
        let mut stmt = self
            .db
            .as_ref()
            .unwrap()
            .prepare("SELECT id, color FROM filaments")?;
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

/// modify, get filament, purge values
impl Db {
    pub fn delete_filament(&mut self, id: u32) -> Result<()> {
        match self.db.as_ref().unwrap().execute(
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
            match self.db.as_ref().unwrap().execute(
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
            match self.db.as_ref().unwrap().execute(
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
        match self.db.as_ref().unwrap().execute(
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
        self.db.as_ref().unwrap().query_row(
            "SELECT purge FROM purge_values WHERE id_from=?1 AND id_to=?2",
            (id_from, id_to),
            |row| row.get(0),
        )
    }

    pub fn get_all_purge_for_filament(&self, id: u32) -> Result<Vec<(u32, (u32, u32))>> {
        let mut stmt1 = self.db.as_ref().unwrap().prepare(
            "SELECT id_from, id_to, purge 
            FROM purge_values 
            WHERE id_from = ?1",
        )?;

        let froms = stmt1.query_map([id], |row| {
            let id_from: u32 = row.get(0)?;
            let id_to: u32 = row.get(1)?;
            let purge: u32 = row.get(2)?;

            assert_eq!(id_from, id);

            Ok((id_from, id_to, purge))
        });

        /// to_id, purge
        let froms: Vec<(u32, u32)> = froms?
            .flatten()
            .map(|(_, to_id, p)| (to_id, p))
            .collect::<Vec<_>>();

        let tos = froms
            .iter()
            .flat_map(|(id_to, _)| self.get_purge_values(*id_to, id).map(|p| (*id_to, p)))
            .collect::<HashMap<_, _>>();

        let mut out = Vec::new();

        for (to_id, p_to) in froms.iter() {
            let p_from = tos.get(to_id).copied().unwrap_or(999);

            out.push((*to_id, (*p_to, p_from)));
        }

        // let mut stmt2 = self.db.prepare(
        //     "SELECT id_from, id_to, purge
        //     FROM purge_values
        //     WHERE id_to = ?1",
        // )?;

        // let tos = stmt2.query_map([id], |row| {
        //     let id_from: u32 = row.get(0)?;
        //     let id_to: u32 = row.get(1)?;
        //     let purge: u32 = row.get(2)?;

        //     assert_eq!(id_to, id);

        //     Ok((id_from, id_to, purge))
        // });

        // Ok(froms?.chain(tos?).flatten().collect())
        Ok(out)
    }
}

#[cfg(feature = "nope")]
impl Db {
    pub fn set_config(&mut self, key: &str, value: &str) -> Result<()> {
        self.db.as_ref().unwrap().execute(
            "INSERT OR REPLACE INTO config (key, value) VALUES (?1, ?2)",
            params![key, value],
        )?;
        Ok(())
    }

    pub fn get_config(&self, key: &str) -> Result<String> {
        self.db.as_ref().unwrap().query_row(
            "SELECT value FROM config WHERE key = ?1",
            [key],
            |row| row.get(0),
        )
    }

    pub fn get_config_or_default(&self, key: &str, default: &str) -> String {
        self.get_config(key).unwrap_or_else(|_| default.to_string())
    }

    pub fn delete_config(&mut self, key: &str) -> Result<()> {
        self.db
            .as_ref()
            .unwrap()
            .execute("DELETE FROM config WHERE key = ?1", [key])?;
        Ok(())
    }

    pub fn get_all_configs(&self) -> Result<HashMap<String, String>> {
        let mut stmt = self
            .db
            .as_ref()
            .unwrap()
            .prepare("SELECT key, value FROM config")?;
        let config_iter = stmt.query_map([], |row| {
            let key: String = row.get(0)?;
            let value: String = row.get(1)?;
            Ok((key, value))
        })?;

        Ok(config_iter.flatten().collect())
    }
}

/// State
impl Db {
    pub fn save_state(&mut self, state: &MinSavedState) -> Result<()> {
        let Ok(s) = ron::ser::to_string(state) else {
            eprintln!("Error serializing state: {:?}", state);
            return Err(rusqlite::Error::InvalidQuery);
        };
        self.db.as_ref().unwrap().execute(
            "INSERT OR REPLACE INTO saved_state (id, state) VALUES (?1, ?2)",
            (0, s),
        )?;
        Ok(())
    }

    pub fn get_state(&self) -> Result<MinSavedState> {
        let s: String = self.db.as_ref().unwrap().query_row(
            "SELECT state FROM saved_state WHERE id = ?1",
            [0],
            |row| row.get(0),
        )?;

        if let Ok(state) = ron::de::from_str::<MinSavedState>(&s) {
            return Ok(state);
        }

        eprintln!("Error deserializing state: {:?}", s);
        Err(rusqlite::Error::InvalidQuery)
    }
}

/// new, reload
impl Db {
    pub fn reload<P: AsRef<std::path::Path>>(&mut self, path: P) -> Result<()> {
        let db = self.db.take().unwrap();
        if let Err(e) = db.close() {
            eprintln!("Error closing db: {:?}", e);
        };

        let conn = Self::_new(path)?;

        self.db = Some(conn);

        Ok(())
    }

    pub fn get_db(&self) -> &Connection {
        self.db.as_ref().unwrap()
    }

    pub fn set_default_black(&mut self, id: u32) -> Result<()> {
        self.db.as_ref().unwrap().execute(
            "INSERT OR REPLACE INTO default_filaments (id, color) VALUES (?1, ?2)",
            (id, "black"),
        )?;
        Ok(())
    }

    pub fn set_default_white(&mut self, id: u32) -> Result<()> {
        self.db.as_ref().unwrap().execute(
            "INSERT OR REPLACE INTO default_filaments (id, color) VALUES (?1, ?2)",
            (id, "white"),
        )?;
        Ok(())
    }

    pub fn get_default_black_white(&self) -> Result<(Option<u32>, Option<u32>)> {
        let mut stmt =
            self.db.as_ref().unwrap().prepare(
                "SELECT id, color FROM default_filaments WHERE color = ?1 OR color = ?2",
            )?;
        let iter = stmt
            .query_map(["black", "white"], |row| {
                let id: u32 = row.get(0)?;
                let color: String = row.get(1)?;
                Ok((id, color))
            })?
            .flatten()
            .collect::<Vec<_>>();

        let mut out_w = None;
        let mut out_b = None;

        for (x, c) in iter {
            if c == "black" {
                out_b = Some(x);
            } else if c == "white" {
                out_w = Some(x);
            }
        }

        Ok((out_b, out_w))
    }

    pub fn new<P: AsRef<std::path::Path>>(path: P) -> Result<Self> {
        let conn = Self::_new(path)?;

        Ok(Self {
            db: Some(conn),
            cache_filaments: (FilamentMap::new(HashMap::new()), vec![]),
            last_updated_filament: std::time::Instant::now(),
            stale_filament: true,

            cache_history: vec![],
            cache_history_sort: None,
            last_updated_history: std::time::Instant::now(),
            stale_history: true,
        })
    }

    fn _new<P: AsRef<std::path::Path>>(path: P) -> Result<Connection> {
        // let path = "test.db";

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

        conn.execute(
            "CREATE TABLE IF NOT EXISTS default_filaments (
          id          INTEGER PRIMARY KEY,
          color       TEXT,
          UNIQUE(id, color)
      )",
            (), // empty list of parameters.
        )?;

        //     conn.execute(
        //         "CREATE TABLE IF NOT EXISTS settings (
        //       key         TEXT PRIMARY KEY,
        //       value       TEXT,
        //       UNIQUE(key, value)
        //   )",
        //         (), // empty list of parameters.
        //     )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS saved_state (
              id          INTEGER PRIMARY KEY,
              state       TEXT
          )",
            (), // empty list of parameters.
        )?;

        Self::init_history(&conn)?;

        Ok(conn)
    }
}
