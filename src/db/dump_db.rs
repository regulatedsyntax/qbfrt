//! Tools for dumping the SQLite database to fastresume files

use crate::common::fastresume::Fastresume;
use crate::config::Config;
use crate::db::db_structs::DatabaseData;
use rusqlite::Connection;
use serde_rusqlite::from_rows;
use std::error::Error;
use std::fs;
use std::path::Path;

/// Dumps the SQLite database to fastresume files
///
/// This should yield a 1:1 re-creation of what qB would generate in the BT_Backup directory. *NOTE:
/// THIS WILL OVERWRITE ANY EXISTING FILES IN THE OUTPUT DIRECTORY!*
///
/// ## Example
/// ```rs
/// use qbfrt::db::dump_fastresume;
/// dump_fastresume::dump(&connection, false);
/// ```
///
/// ## Configuration
/// - You can configure the output directory by setting config.output_directory, otherwise it defaults
///   to a `qbfrt_dump` directory in the current working directory.
///
/// ## Verbose output
/// If verbose output is enabled it will output the torrent hash for each fastresume file created.
pub fn to_fastresume(db: &Connection, config: &Config) -> Result<(), Box<dyn Error>> {
    println!("DB -> fastresume: creating fastresume files...");

    let mut num_torrents_dumped = 0;

    let dir_path = config
        .output_directory
        .as_deref()
        .map(Path::new)
        .unwrap_or_else(|| Path::new("qbfrt_dump"));
    fs::create_dir_all(dir_path)?;
    if config.verbose {
        println!("DB -> fastresume: output directory: {:?}", dir_path);
    }

    let mut search_stmt = db.prepare("SELECT * FROM torrents")?;
    let all_torrents = from_rows::<DatabaseData>(search_stmt.query([])?);

    for torrent in all_torrents {
        let torrent = match torrent {
            Ok(torrent) => torrent,
            Err(err) => {
                eprintln!("DB -> fastresume: Skipping item due to error: {err}");
                continue;
            }
        };

        let bencoded_resume_data = torrent.libtorrent_resume_data.as_slice();
        let mut resume_data =
            serde_bencode::from_bytes::<Fastresume>(bencoded_resume_data).unwrap();

        // qB-specific data are stored in the database columns, so we need to add them back to the fastresume blob
        resume_data.qbt_category = Some(torrent.category.unwrap_or_default().into_bytes());
        resume_data.qbt_content_layout = Some(torrent.content_layout.into_bytes());
        resume_data.qbt_first_last_piece_priority = Some(torrent.has_outer_pieces_priority);
        resume_data.qbt_inactive_seeding_time_limit = Some(torrent.inactive_seeding_time_limit);
        resume_data.qbt_name = Some(torrent.name.unwrap_or_default());
        resume_data.qbt_ratio_limit = Some(torrent.ratio_limit);
        resume_data.qbt_seed_status = Some(torrent.has_seed_status);
        resume_data.qbt_seeding_time_limit = Some(torrent.seeding_time_limit);
        resume_data.qbt_share_limit_action = Some(torrent.share_limit_action.unwrap_or_default());
        resume_data.qbt_stop_condition = Some(torrent.stop_condition);

        // Paths are absent from the .fastresume if the torrent is in "AutoTMM" mode
        if torrent.target_save_path.is_some() {
            resume_data.qbt_download_path = Some(torrent.download_path.unwrap_or_default());
            resume_data.qbt_save_path = Some(torrent.target_save_path.unwrap_or_default());
        }

        // Tags are comma-separated in the database, but a list in the fastresume file
        match torrent.tags {
            Some(tags) => {
                let tags: Vec<String> = tags.split(',').map(|s| s.to_string()).collect();
                resume_data.qbt_tags = Some(tags);
            }
            // If there are no tags we need to set an empty list
            None => resume_data.qbt_tags = Some(vec![]),
        }

        let new_resume_data = serde_bencode::to_bytes(&resume_data)?;

        let fastresume_file_name = format!("{}.fastresume", torrent.torrent_id);
        let fastresume_file = dir_path.join(fastresume_file_name);
        let torrent_file_name = format!("{}.torrent", torrent.torrent_id);
        let torrent_file = dir_path.join(torrent_file_name);
        fs::write(fastresume_file, new_resume_data)?;
        fs::write(torrent_file, torrent.metadata.as_slice())?;

        if config.verbose {
            println!(
                "DB -> fastresume: fastresume created for {}",
                torrent.torrent_id
            );
        }

        num_torrents_dumped += 1;
    }

    match num_torrents_dumped {
        0 => println!("DB -> fastresume: no torrents were dumped"),
        1 => println!("DB -> fastresume: 1 torrent was dumped"),
        _ => println!(
            "DB -> fastresume: {} torrents were dumped",
            num_torrents_dumped
        ),
    }

    Ok(())
}
