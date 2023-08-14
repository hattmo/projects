use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
pub struct ServerSettings {
    name: String,
    description: String,
    tags: Vec<String>,
    max_players: usize,
    visibility: Visibility,
    username: String,
    password: String,
    token: String,
    game_password: String,
    require_user_verification: bool,
    max_upload_in_kilobytes_per_second: usize,
    max_upload_slots: usize,
    minimum_latency_in_ticks: usize,
    max_heartbeats_per_second: usize,
    ignore_player_limit_for_returning_players: bool,
    allow_commands: String,
    autosave_interval: usize,
    autosave_slots: usize,
    afk_autokick_interval: usize,
    auto_pause: bool,
    only_admins_can_pause_the_game: bool,
    autosave_only_on_server: bool,
    non_blocking_saving: bool,
    minimum_segment_size: usize,
    minimum_segment_size_peer_count: usize,
    maximum_segment_size: usize,
    maximum_segment_size_peer_count: usize,
}

#[derive(Deserialize, Serialize, Default)]
struct Visibility {
    public: bool,
    lan: bool,
}
