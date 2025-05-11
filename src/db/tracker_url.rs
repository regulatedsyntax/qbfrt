//! Tools for modifying torrent tracker URLs

use crate::common::fastresume::Fastresume;
use crate::config::Config;
use crate::db::db_structs::FetchedLibtorrentResumeData;
use rusqlite::{named_params, Connection};
use serde_rusqlite::from_rows;
use std::error::Error;

/// Tracker url information
#[derive(Debug)]
pub struct TrackerUrl {
    /// Existing tracker URL
    pub old: String,
    /// New tracker URL
    pub new: String,
}

/// Performs a string replace operation on torrent trackers
///
/// ## Example
/// ```rs
/// use qbfrt::db::{tracker_url, TrackerUrl};
/// let tracker_url = TrackerUrl {
///     old: String::from("http://"),
///     new: String::from("https://"),
/// };
/// change_tracker_url(&connection, tracker_url, false);
/// ```
///
/// ## Verbose output
/// If verbose output is enabled it will output the torrent hash and full trackers list for
/// the updated torrent.
pub fn change_tracker_url(
    db: &Connection,
    tracker_url: &TrackerUrl,
    config: &Config,
) -> Result<(), Box<dyn Error>> {
    println!(
        "Tracker url: replacing '{}' with '{}'",
        tracker_url.old, tracker_url.new
    );

    // First we fetch every torrent and iterate through it to see if it needs an update
    let mut search_stmt = db.prepare(
        "SELECT id, torrent_id, libtorrent_resume_data
        FROM torrents",
    )?;
    let all_torrents = from_rows::<FetchedLibtorrentResumeData>(search_stmt.query([])?);

    let mut num_updated = 0;
    for row in all_torrents {
        let torrent = row?;
        let bencoded_data = torrent.libtorrent_resume_data.as_slice();
        let mut libtorrent_resume_data: Fastresume = serde_bencode::from_bytes(bencoded_data)?;

        let mut trigger_update = false;
        // Trackers are stored in a nested bencode list
        let mut updated_trackers: Vec<Vec<String>> = Vec::new();
        libtorrent_resume_data.trackers.iter().for_each(|outer| {
            let mut tracker_list: Vec<String> = Vec::new();

            outer.iter().for_each(|tracker| {
                // this will be used later to trigger update on only relevant torrents
                if tracker.contains(&tracker_url.old) {
                    trigger_update = true;
                }

                tracker_list.push(tracker.replace(&tracker_url.old, &tracker_url.new));
            });

            updated_trackers.push(tracker_list);
        });

        libtorrent_resume_data.trackers = updated_trackers;

        if trigger_update {
            let mut update_stmt = db.prepare(
                "UPDATE torrents
                SET libtorrent_resume_data = :lrd
                WHERE id = :id
                RETURNING torrent_id;",
            )?;
            update_stmt.query_row(named_params! {":lrd": serde_bencode::to_bytes(&libtorrent_resume_data)?, ":id": torrent.id}, |row| {
                let updated_row_id = row.get::<usize, String>(0)?;

                if config.verbose {
                    println!("Tracker url: updated tracker URLs for {}", updated_row_id);
                    println!("{}: new tracker urls are {:?}", updated_row_id, libtorrent_resume_data.trackers);
                }

                num_updated += 1;

                Ok(())
            })?;
        }
    }

    match num_updated {
        0 => print!("Tracker url: no torrents were updated"),
        1 => println!("Tracker url: 1 torrent was updated"),
        _ => println!("Tracker url: {} torrents were updated", num_updated),
    }

    Ok(())
}
