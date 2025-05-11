//! qbfrt configuration module
//!
//! Configures the application based on the passed command line arguments

use crate::db::save_path::SavePath;
use crate::db::tracker_url::TrackerUrl;
use argh::FromArgs;
use core::panic;
use directories::BaseDirs;
use std::path::{Path, PathBuf, MAIN_SEPARATOR};

/// Returns the OS-specific qB .torrent data directory
fn get_qb_dir() -> PathBuf {
    let base_dirs = BaseDirs::new().unwrap();
    base_dirs.data_local_dir().join("qBittorrent")
}

/// CLI argument options
#[derive(Debug, FromArgs)]
struct CLIOpts {
    /// path to qB local config directory (where torrents.db lives)
    #[argh(option, short = 'p')]
    config_dir: Option<String>,
    /// disable automatic torrents.db backup
    #[argh(switch, short = 'd')]
    disable_backup: bool,
    /// enable verbose output
    #[argh(switch, short = 'v')]
    verbose: bool,
    /// path string to replace
    #[argh(option)]
    old_path: Option<String>,
    /// new path string
    #[argh(option)]
    new_path: Option<String>,
    /// force using path slash '/' separators
    #[argh(switch)]
    use_unix_sep: bool,
    /// force using Windows backslash '\' separators
    #[argh(switch)]
    use_win_sep: bool,
    /// tracker string to replace
    #[argh(option)]
    old_tracker: Option<String>,
    /// new tracker string
    #[argh(option)]
    new_tracker: Option<String>,
    /// extract fastresume files
    #[argh(switch)]
    db_to_fastresume: bool,
    /// output directory for fastresume files
    #[argh(option, short = 'o')]
    output_dir: Option<String>,
}

/// Application configuration generated from CLI arguments
#[derive(Debug)]
pub struct Config {
    /// Path to qB local config directory (where torrents.db lives)
    pub qb_directory: PathBuf,
    /// The full path to torrents.db file
    pub db_file: PathBuf,
    /// Disables the automatic torrents.db backup
    pub disable_backup: bool,
    /// Torrent save path information
    pub save_path: Option<SavePath>,
    /// Torrent tracker url information
    pub tracker_url: Option<TrackerUrl>,
    /// Dumps the database to fastresume files
    pub db_to_fastresume: bool,
    /// The output directory for fastresume files
    pub output_directory: Option<String>,
    /// Toggles verbose output
    pub verbose: bool,
}

impl Config {
    /// Builds application configuration using command line arguments
    ///
    /// ## Example
    /// ```rs
    /// let config = Config::build().unwrap_or_else(|err| {
    ///     println!("Problem parsing arguments: {err}");
    ///     process::exit(1);
    /// });
    /// ```
    pub fn build() -> Result<Config, String> {
        let args: CLIOpts = argh::from_env();
        let qb_dir = get_qb_dir();

        let qb_directory = match args.config_dir {
            Some(dir) => PathBuf::from(&dir),
            _ => qb_dir,
        };

        let db_file = Path::new(&qb_directory).join("torrents.db");

        let save_path = match (args.old_path, args.new_path) {
            (Some(old), Some(new)) => {
                let separator: String;
                if args.use_unix_sep {
                    separator = '/'.to_string();
                } else if args.use_win_sep {
                    separator = '\\'.to_string();
                } else {
                    separator = MAIN_SEPARATOR.to_string();
                }

                // qB saves a Unix-style path irrespective of OS, so we need to use that in some places
                // Yes, even Windows network paths are saved that way e.g. \\server\folder -> //server/folder
                let old_unix = old.replace('\\', "/");
                let new_unix = new.replace('\\', "/");

                // qB saves the path in a second location with OS-specific separators. It will be up to the user
                // to pick the appropriate separator for the existing path to replace
                Some(SavePath {
                    old_unix,
                    new_unix,
                    old,
                    new,
                    separator,
                })
            }
            (None, None) => None,
            (Some(_old), None) => panic!("--new-path is missing!"),
            (None, Some(_new)) => panic!("--old-path is missing!"),
        };

        let tracker_url = match (args.old_tracker, args.new_tracker) {
            (Some(old), Some(new)) => Some(TrackerUrl { old, new }),
            (None, None) => None,
            (Some(_old), None) => panic!("--new-tracker is missing!"),
            (None, Some(_new)) => panic!("--old-tracker is missing!"),
        };

        let config = Config {
            qb_directory,
            db_file,
            disable_backup: args.disable_backup,
            save_path,
            tracker_url,
            db_to_fastresume: args.db_to_fastresume,
            output_directory: args.output_dir,
            verbose: args.verbose,
        };

        if config.verbose {
            println!("Verbose output enabled");
            println!("Using {:?} as qB directory", config.qb_directory.display());
            println!("Using {:?} as qB database", config.db_file.display());
            println!("Save path: {:?}", config.save_path);
            println!("Tracker url: {:?}", config.tracker_url);
        }

        Ok(config)
    }
}
