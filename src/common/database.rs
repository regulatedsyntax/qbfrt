//! SQLite file data structures

use serde_derive::{Deserialize, Serialize};

/// qB SQLite database row data
///
/// Each field corresponds to a column in the "torrents" database table
#[derive(Serialize, Deserialize, Debug)]
pub struct DatabaseRow {
    id: u64,
    /// Torrent hash
    pub torrent_id: String,
    queue_position: i64,
    /// Torrent name, only used on re-named torrents
    pub name: Option<String>,
    /// Torrent category
    pub category: Option<String>,
    /// Comma-separated list of tags
    pub tags: Option<String>,
    /// Save path for torrent content
    ///
    /// This is absent if the torrent is in 'AutoTMM' mode
    pub target_save_path: Option<String>,
    /// Download path, used for incomplete download directory
    pub download_path: Option<String>,
    /// Torrent content layout, whether sub-folders are created or not
    pub content_layout: String,
    /// Ratio limit for seeding
    pub ratio_limit: i64,
    /// Time limit for seeding
    pub seeding_time_limit: i64,
    /// Inactivity time limit for seeding
    pub inactive_seeding_time_limit: i64,
    /// Action to take when share limit is reached
    pub share_limit_action: Option<String>,
    /// Prioritize outer (first and last) pieces
    pub has_outer_pieces_priority: i64,
    /// Torrent is seeding
    pub has_seed_status: i64,
    operating_mode: String,
    stopped: i64,
    /// Stop condition for torrents
    pub stop_condition: String,
    /// Binary blob containing libtorrent fastresume data
    ///
    /// See "common::fastresume::Fastresume" for deserialized contents
    pub libtorrent_resume_data: Vec<u8>,
    /// Binary blob containing metadata
    pub metadata: Vec<u8>,
}

/// A subset of data for save path operations
#[derive(Serialize, Deserialize, Debug)]
pub struct PathData {
    /// Torrent database id
    pub id: u64,
    /// Torrent hash
    pub torrent_id: String,
    /// Save path for torrent content
    pub target_save_path: Option<String>,
    /// Binary blob containing fastresume data
    pub libtorrent_resume_data: Vec<u8>,
}

/// A subset of data for operations on libtorrent_resume_data
#[derive(Serialize, Deserialize, Debug)]
pub struct LibtorrentResumeData {
    /// Torrent database row id
    pub id: u64,
    /// Torrent hash id
    pub torrent_id: String,
    ///Binary blob containing fastresume data
    pub libtorrent_resume_data: Vec<u8>,
}
