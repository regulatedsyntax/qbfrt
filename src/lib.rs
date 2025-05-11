//! # qbfrt (qBittorrent fastresume tool)
//! Command line tool for working with qBittorrent's fastresume data. Supports the
//! experimental SQLite database and (soon) traditional .fastresume files.
//!
//! ## Features
//! With this tool you can:
//! - Mass update the save paths for torrents in the SQLite database
//!     - Change files to a new drive or directory without having to move torrents in
//!       qBittorrent or recheck all of the torrent data
//!     - Migrate from qBittorrent on Windows to Linux without having to recheck the
//!       torrent data
//! - Mass update the tracker URLs for torrents in the SQLite database
//! - Dump the SQLite database to .fastresume files (1:1 re-creation of what qB would
//!   generate in the BT_Backup directory)
//!
//! **More functionality to come!**
//!
//! ## Note
//! You can chain multiple tasks together, such as changing a tracker and a save path at
//! the same time. `--db-to-fastresume` runs after other commands.

#![warn(missing_docs)]

pub mod common;
pub mod config;
pub mod db;
