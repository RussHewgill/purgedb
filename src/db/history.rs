// use anyhow::{anyhow, bail, ensure, Context, Result};
use tracing::{debug, error, info, trace, warn};

use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, Result};

use crate::gui::history_tab::HistoryRow;

use super::Db;

impl Db {
    pub fn init_history(conn: &Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS send_history (
          id            INTEGER PRIMARY KEY,
          timestamp     INTEGER NOT NULL,
          num_slots     INTEGER NOT NULL,
          slot1         INTEGER,
          slot2         INTEGER,
          slot3         INTEGER,
          slot4         INTEGER,
          slot5         INTEGER,
          slot6         INTEGER,
          slot7         INTEGER,
          slot8         INTEGER,
          slot9         INTEGER,
          slot10        INTEGER,
          slot11        INTEGER,
          slot12        INTEGER,
          slot13        INTEGER,
          slot14        INTEGER,
          slot15        INTEGER,
          slot16        INTEGER,
          multiplier    FLOAT,
          offset        FLOAT
      )",
            //   filament_grid BLOB NOT NULL
            (), // empty list of parameters.
        )?;

        Ok(())
    }

    pub fn add_to_history(
        &mut self,
        grid: &crate::gui::filament_grid::FilamentGridData,
    ) -> Result<()> {
        // debug!("saving history: {:?}", grid);
        debug!("saving history");

        // let ps = params![]

        match self.db.execute(
            "INSERT INTO send_history (timestamp, num_slots, slot1, slot2, slot3, slot4, slot5, slot6, slot7, slot8, slot9, slot10, slot11, slot12, slot13, slot14, slot15, slot16, multiplier, offset) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20)",
            //  #[cfg(feature = "nope")]
            params![
                chrono::Utc::now().timestamp(),
                grid.num_filaments() as i64,
                grid.id(0),
                grid.id(1),
                grid.id(2),
                grid.id(3),
                grid.id(4),
                grid.id(5),
                grid.id(6),
                grid.id(7),
                grid.id(8),
                grid.id(9),
                grid.id(10),
                grid.id(11),
                grid.id(12),
                grid.id(13),
                grid.id(14),
                grid.id(15),
                grid.multiplier as f64,
                grid.offset as f64,
            ],
        ) {
            Ok(_) => (),
            // Err(e) => eprintln!("e = {:?}", e),
            Err(e) => {}
        }

        self.stale_history = true;
        Ok(())
    }

    pub fn is_stale_history(&self) -> bool {
        self.stale_history || self.last_updated_history.elapsed() > super::CACHE_DURATION
    }

    fn sort_history_cache(&mut self, sort: Option<(usize, crate::gui::history_tab::SortOrder)>) {
        match sort {
            None | Some((0, crate::gui::history_tab::SortOrder::Ascending)) => {
                self.cache_history.sort_by_key(|r| r.timestamp);
            }
            Some((0, crate::gui::history_tab::SortOrder::Descending)) => {
                self.cache_history.sort_by_key(|r| r.timestamp);
                self.cache_history.reverse();
            }
            _ => (),
        }
        self.cache_history_sort = sort;
    }

    pub fn remove_history(&mut self, id: u32) -> Result<()> {
        self.db
            .execute("DELETE FROM send_history WHERE id = ?1", params![id])?;
        self.stale_history = true;
        Ok(())
    }

    pub fn fetch_history(
        &mut self,
        sort: Option<(usize, crate::gui::history_tab::SortOrder)>,
    ) -> Result<Vec<HistoryRow>> {
        // debug!("fetch_history");
        if !self.is_stale_history() {
            if sort == self.cache_history_sort {
                // debug!("returning cached history 0");
                return Ok(self.cache_history.clone());
            } else {
                // debug!("returning cached history 1");
                self.sort_history_cache(sort);
                return Ok(self.cache_history.clone());
            }
        }
        debug!("reading history from db");

        let mut stmt = {
            "SELECT
            id,
            timestamp,
            num_slots,
            slot1,
            slot2,
            slot3,
            slot4,
            slot5,
            slot6,
            slot7,
            slot8,
            slot9,
            slot10,
            slot11,
            slot12,
            slot13,
            slot14,
            slot15,
            slot16,
            multiplier,
            offset
      FROM send_history
      "
            .to_string()
        };

        match sort {
            Some((0, crate::gui::history_tab::SortOrder::Ascending)) => {
                stmt.push_str(&format!("ORDER BY timestamp ASC"));
            }
            Some((0, crate::gui::history_tab::SortOrder::Descending)) => {
                stmt.push_str(&format!("ORDER BY timestamp DESC"));
            }
            _ => (),
        }

        let mut stmt = self.db.prepare(&stmt)?;

        let iter = stmt.query_map([], |row| {
            let id = match row.get(0) {
                Ok(id) => id,
                Err(e) => {
                    error!("row.get(0) = {:?}", e);
                    panic!("row.get(0) = {:?}", e);
                }
            };

            // eprintln!("row = {:?}", row);
            let timestamp: i64 = row.get(1)?;
            let timestamp: DateTime<Utc> =
                DateTime::from_timestamp(timestamp, 0).unwrap_or_default();

            let num_filaments = row.get(2)?;

            let mut filaments = vec![];

            for i in 0..num_filaments {
                filaments.push(row.get(i + 3)?);
            }

            let multiplier = match row.get(19) {
                Ok(multiplier) => Some(multiplier),
                Err(e) => None,
            };
            let offset = match row.get(20) {
                Ok(offset) => Some(offset),
                Err(e) => None,
            };

            Ok(HistoryRow {
                id,
                timestamp,
                num_filaments,
                filaments,
                multiplier,
                offset,
            })
        })?;

        let out = iter.flatten().collect::<Vec<_>>();
        drop(stmt);

        self.cache_history = out;
        self.sort_history_cache(sort);

        self.stale_history = false;
        self.last_updated_history = std::time::Instant::now();

        Ok(self.cache_history.clone())
    }
}
