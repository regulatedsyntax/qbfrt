//! Fastresume file data structures

use serde_derive::{Deserialize, Serialize};

/// Fastresume file data
#[derive(Serialize, Deserialize, Debug)]
pub struct Fastresume {
    active_time: i64,
    added_time: i64,
    #[serde(with = "serde_bytes")]
    allocation: Vec<u8>,
    apply_ip_filter: u8,
    auto_managed: u8,
    #[serde(default, with = "serde_bytes")]
    banned_peers: Option<Vec<u8>>,
    #[serde(default, with = "serde_bytes")]
    banned_peers6: Option<Vec<u8>>,
    completed_time: i64,
    disable_dht: u8,
    disable_lsd: u8,
    disable_pex: u8,
    download_rate_limit: i32,
    #[serde(rename = "file-format")]
    #[serde(with = "serde_bytes")]
    file_format: Vec<u8>,
    #[serde(rename = "file-version")]
    file_version: u8,
    file_priority: Option<Vec<u8>>,
    finished_time: i64,
    httpseeds: Option<Vec<String>>,
    i2p: Option<u8>,
    #[serde(rename = "info-hash")]
    #[serde(with = "serde_bytes")]
    info_hash: Vec<u8>,
    #[serde(rename = "info-hash2")]
    #[serde(default, with = "serde_bytes")]
    info_hash2: Option<Vec<u8>>,
    last_download: i64,
    last_seen_complete: i64,
    last_upload: i64,
    #[serde(rename = "libtorrent-version")]
    #[serde(with = "serde_bytes")]
    libtorrent_version: Vec<u8>,
    max_connections: i64,
    max_uploads: i64,
    #[serde(default, with = "serde_bytes")]
    name: Option<Vec<u8>>,
    num_complete: u64,
    num_downloaded: u64,
    num_incomplete: u64,
    paused: u8,
    #[serde(default, with = "serde_bytes")]
    peers: Option<Vec<u8>>,
    #[serde(default, with = "serde_bytes")]
    peers6: Option<Vec<u8>>,
    #[serde(default, with = "serde_bytes")]
    piece_priority: Option<Vec<u8>>,
    #[serde(with = "serde_bytes")]
    pieces: Vec<u8>,
    /// Torrent category
    #[serde(rename = "qBt-category")]
    #[serde(default, with = "serde_bytes")]
    pub qbt_category: Option<Vec<u8>>,
    /// Content layout mode for torrent
    #[serde(rename = "qBt-contentLayout")]
    #[serde(default, with = "serde_bytes")]
    pub qbt_content_layout: Option<Vec<u8>>,
    /// Download path for torrent
    #[serde(rename = "qBt-downloadPath")]
    pub qbt_download_path: Option<String>,
    /// Toggle prioritization of first and last pieces
    #[serde(rename = "qBt-firstLastPiecePriority")]
    pub qbt_first_last_piece_priority: Option<i64>,
    /// Inactive seeding time limit for torrent
    #[serde(rename = "qBt-inactiveSeedingTimeLimit")]
    pub qbt_inactive_seeding_time_limit: Option<i64>,
    /// Torrent name
    ///
    /// Is only set if the torrent was renamed
    #[serde(rename = "qBt-name")]
    pub qbt_name: Option<String>,
    /// Ratio limit for torrent
    #[serde(rename = "qBt-ratioLimit")]
    pub qbt_ratio_limit: Option<i64>,
    /// Save path for torrent
    #[serde(rename = "qBt-savePath")]
    pub qbt_save_path: Option<String>,
    /// Seeding status for torrent, i.e. is it seeding
    #[serde(rename = "qBt-seedStatus")]
    pub qbt_seed_status: Option<i64>,
    /// Seeding time limit for torrent
    #[serde(rename = "qBt-seedingTimeLimit")]
    pub qbt_seeding_time_limit: Option<i64>,
    /// Action to take when share limit is reached
    #[serde(rename = "qBt-shareLimitAction")]
    pub qbt_share_limit_action: Option<String>,
    /// Stop condition for torrent
    #[serde(rename = "qBt-stopCondition")]
    pub qbt_stop_condition: Option<String>,
    /// List of tags for torrent
    #[serde(rename = "qBt-tags")]
    pub qbt_tags: Option<Vec<String>>,
    #[serde(with = "serde_bytes")]
    /// Save path for torrent
    pub save_path: Vec<u8>,
    seed_mode: u8,
    seeding_time: i64,
    sequential_download: u8,
    share_mode: u8,
    stop_when_ready: u8,
    super_seeding: u8,
    total_downloaded: u64,
    total_uploaded: u64,
    /// Trackers list for torrent
    pub trackers: Vec<Vec<String>>,
    unfinished: Option<Vec<UnfinishedPiece>>,
    upload_mode: u8,
    upload_rate_limit: i64,
    #[serde(rename = "url-list")]
    url_list: Vec<String>,
}

/// Fastresume unfinished piece data
#[derive(Serialize, Deserialize, Debug)]
pub struct UnfinishedPiece {
    #[serde(with = "serde_bytes")]
    bitmask: Vec<u8>,
    piece: i64,
}
