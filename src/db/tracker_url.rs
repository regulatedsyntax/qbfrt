//! Tools for modifying torrent tracker URLs

use crate::common::database::LibtorrentResumeData;
use crate::common::fastresume::Fastresume;
use crate::config::Config;
use crate::db::query;
use rusqlite::{named_params, Connection};
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
/// let config = Config { verbose: true };
/// let tracker_url = TrackerUrl {
///     old: String::from("http://"),
///     new: String::from("https://"),
/// };
/// change_tracker_url(&connection, tracker_url, config);
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

    let all_torrents = query::fetch_all_torrents::<LibtorrentResumeData>(
        db,
        "
        SELECT id, torrent_id, libtorrent_resume_data 
        FROM torrents
        ",
    )?;

    let mut num_torrents_updated = 0;
    for torrent in all_torrents {
        let mut trigger_update = false;
        let bencoded_resume_data = torrent.libtorrent_resume_data.as_slice();
        let mut resume_data: Fastresume = serde_bencode::from_bytes(bencoded_resume_data)?;

        // Trackers are stored in a nested bencode list
        let mut updated_trackers: Vec<Vec<String>> = Vec::new();
        resume_data.trackers.iter().for_each(|outer| {
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

        resume_data.trackers = updated_trackers;

        if trigger_update {
            let mut update_stmt = db.prepare(
                "
                UPDATE torrents
                SET libtorrent_resume_data = :lrd
                WHERE id = :id
                RETURNING torrent_id;
                ",
            )?;
            update_stmt.query_row(
                named_params! {":lrd": serde_bencode::to_bytes(&resume_data)?, ":id": torrent.id},
                |row| {
                    let updated_row_id = row.get::<usize, String>(0)?;

                    if config.verbose {
                        println!("Tracker url: updated tracker URLs for {}", updated_row_id);
                        println!(
                            "{}: new tracker urls are {:?}",
                            updated_row_id, resume_data.trackers
                        );
                    }

                    num_torrents_updated += 1;

                    Ok(())
                },
            )?;
        }
    }

    match num_torrents_updated {
        0 => print!("Tracker url: no torrents were updated"),
        1 => println!("Tracker url: 1 torrent was updated"),
        _ => println!(
            "Tracker url: {} torrents were updated",
            num_torrents_updated
        ),
    }

    Ok(())
}
