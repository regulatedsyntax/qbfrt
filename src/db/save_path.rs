//! Tools for modifying torrent save path

use crate::common::database::PathData;
use crate::common::fastresume::Fastresume;
use crate::config::Config;
use crate::db::query;
use rusqlite::{named_params, Connection};
use std::error::Error;

/// Fastresume save path information
///
/// qB stores the torrent save path in two database locations: the `target_save_path` field and in the
/// `libtorrent_fastresume_data` blob. target_save_path uses Unix paths regardless of OS, while libtorrent_fastresume_data
/// uses OS-specific paths. Thus, we have to configure both style of paths.
#[derive(Debug)]
pub struct SavePath {
    /// Existing save path string, Unix-style
    pub old_unix: String,
    /// New save path string, Unix-style
    pub new_unix: String,
    /// Existing save path string, OS-specific
    pub old: String,
    /// New save path string, OS-specific
    pub new: String,
    /// Separator to use in file paths, should default to current OS style
    pub separator: String,
}

/// Performs a string replace operation on two database columns where qB stores the
/// save path information: `target_save_path` and `libtorrent_resume_data`.
///
/// ## Example
/// ```rs
/// use qbfrt::db::save_path::{change_save_path, SavePath};
/// let config = Config { verbose: true };
/// let save_path = SavePath {
///     old_unix: String::from("/old/save/path"),
///     new_unix: String::from("/new/test/dir"),
///     old: String::from("\\old\\save\\path"),
///     new: String::from("\\new\\test\\dir"),
///     separator: '\\'.to_string(),
/// };
/// change_save_path(&connection, save_path, config);
/// ```
///
/// ## Verbose output
/// If verbose output is enabled it will output the torrent hash, the new target_save_path,
/// and the new save_path within in the libtorrent_resume_data blob.
///
/// ## Troubleshooting
/// ### Save path was updated but qB says the torrent files are missing
/// > Make sure your replacement string was correct. Open the database with something such as
/// > [Beekeeper Studio](https://www.beekeeperstudio.io/) and look at the target_save_path column
/// > in the torrents table. You can use an online hex to string converter to look at the
/// > libtorrent_resume_data save path. It should be the same as the target_save_path field.
///
/// ### The target_save_path changed but libtorrent_resume_data did not
/// > Check if your torrent is in Automatic Torrent Management mode (AutoTMM), because in that case the target_save_path column will be null.
/// > Restore the old database and re-run the command with verbose output enabled. Make sure that.
/// > the target_save_path and libtorrent_resume_data have the exact same path separators for the new.
/// > string. If they are different, you likely used the incorrect path separators in the old string.
pub fn change_save_path(
    db: &Connection,
    save_path: &SavePath,
    config: &Config,
) -> Result<(), Box<dyn Error>> {
    println!(
        "Save path: replacing {} with {}",
        save_path.old, save_path.new
    );

    let all_torrents = query::fetch_all_torrents::<PathData>(
        db,
        "
        SELECT id, torrent_id, target_save_path, libtorrent_resume_data 
        FROM torrents
        ",
    )?;

    let mut num_torrents_updated = 0;
    for torrent in all_torrents {
        let mut trigger_update = false;
        let bencoded_resume_data = torrent.libtorrent_resume_data.as_slice();
        let mut resume_data: Fastresume = serde_bencode::from_bytes(bencoded_resume_data)?;
        let resume_data_save_path = String::from_utf8(resume_data.save_path)?;

        // libtorrent save path is the source of truth since it will always be defined
        if resume_data_save_path.contains(&save_path.old) {
            trigger_update = true;
        }

        // qB stores the "target_save_path" with Unix-style separators, even on Windows
        // This field will be absent if the torrent is in AutoTMM mode
        let mut target_save_path: Option<String> = None;
        if torrent.target_save_path.is_some() {
            target_save_path = Some(
                torrent
                    .target_save_path
                    .unwrap()
                    .replace(&save_path.old_unix, &save_path.new_unix),
            );
        }

        // In the libtorrent data, qB uses OS-specific separators. It is up to the end user to make
        // sure their path strings use the appropriate separator for matching. However, we do allow
        // conversion to and from Windows- and Unix-style separators after the replacement.
        if save_path.separator == *"\\" {
            resume_data.save_path = resume_data_save_path
                .replace(&save_path.old, &save_path.new)
                .replace('/', &save_path.separator)
                .into();
        } else {
            resume_data.save_path = resume_data_save_path
                .replace(&save_path.old, &save_path.new)
                .replace('\\', &save_path.separator)
                .into();
        }

        if trigger_update {
            let mut update_stmt = db.prepare(
                "
                UPDATE torrents
                SET target_save_path = :tsp, libtorrent_resume_data = :lrd
                WHERE id = :id
                RETURNING torrent_id;
                ",
            )?;
            update_stmt.query_row(
                named_params! {":tsp": target_save_path, ":lrd": serde_bencode::to_bytes(&resume_data)?, ":id": torrent.id},
                |row| {
                    let updated_row_id = row.get::<usize, String>(0)?;

                    if config.verbose {
                        println!("Save path: updated save path for {}", updated_row_id);
                        println!("{}: new target_save_path is '{:?}'", updated_row_id, target_save_path);
                        println!("{}: new libtorrent_resume_data path is {:?}", updated_row_id, String::from_utf8(resume_data.save_path).unwrap());
                    }

                    num_torrents_updated +=1;

                    Ok(())
                }
            )?;
        }
    }

    match num_torrents_updated {
        0 => println!("Save path: no torrents were updated"),
        1 => println!("Save path: 1 torrent was updated"),
        _ => println!("Save path: {} torrents were updated", num_torrents_updated),
    }

    Ok(())
}
