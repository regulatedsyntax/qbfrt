//! Tools for querying the SQLite database

use rusqlite::{Connection, Result};
use serde::de::DeserializeOwned;
use serde_rusqlite::{from_rows, Error as SerdeRusqliteError};

/// Fetches all torrents from the database
///
/// ## Example
/// ```rs
/// let all_torrents = DB::fetch_all_torrents(&db, "SELECT * from torrents").unwrap_or_else(|err| {
///    println!("Could not fetch torrents: {err}");
///    process::exit(1);
/// });
/// ```
pub fn fetch_all_torrents<T: DeserializeOwned>(
    db: &Connection,
    query_statement: &str,
) -> Result<Vec<T>, SerdeRusqliteError> {
    let mut stmt = db.prepare(query_statement)?;
    let rows = from_rows::<T>(stmt.query([])?);
    rows.collect()
}
