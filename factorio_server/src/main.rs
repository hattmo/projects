use serde::{Deserialize, Serialize};
use tokio::process::Command;

#[derive(Deserialize, Serialize)]
struct Visibility {
    public: bool,
    lan: bool,
}

#[derive(Deserialize, Serialize)]
struct ServerSettings {
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

#[derive(Deserialize, Serialize)]
struct DifficultySettings {
    recipe_difficulty: usize,
    technology_difficulty: usize,
    technology_price_multiplier: usize,
    research_queue_setting: String,
}

#[derive(Deserialize, Serialize)]
struct Pollution {
    enabled: bool,
    diffusion_ratio: f32,
    min_to_diffuse: usize,
    ageing: usize,
    expected_max_per_chunk: usize,
    min_to_show_per_chunk: usize,
    min_pollution_to_damage_trees: usize,
    pollution_with_max_forest_damage: usize,
    pollution_per_tree_damage: usize,
    pollution_restored_per_tree_damage: usize,
    max_pollution_to_restore_trees: usize,
    enemy_attack_pollution_consumption_modifier: usize,
}

#[derive(Deserialize, Serialize)]
struct EnemyEvolution {
    enabled: bool,
    time_factor: f32,
    destroy_factor: f32,
    pollution_factor: f32,
}

#[derive(Deserialize, Serialize)]
struct EnemyExpansion {
    enabled: bool,
    min_base_spacing: usize,
    max_expansion_distance: usize,
    friendly_base_influence_radius: usize,
    enemy_building_influence_radius: usize,
    building_coefficient: f32,
    other_base_coefficient: f32,
    neighbouring_chunk_coefficient: f32,
    neighbouring_base_chunk_coefficient: f32,
    max_colliding_tiles_coefficient: f32,
    settler_group_min_size: usize,
    settler_group_max_size: usize,
    min_expansion_cooldown: usize,
    max_expansion_cooldown: usize,
}

#[derive(Deserialize, Serialize)]
struct UnitGroup {
    min_group_gathering_time: usize,
    max_group_gathering_time: usize,
    max_wait_time_for_late_members: usize,
    max_group_radius: f32,
    min_group_radius: f32,
    max_member_speedup_when_behind: f32,
    max_member_slowdown_when_ahead: f32,
    max_group_slowdown_factor: f32,
    max_group_member_fallback_factor: usize,
    member_disown_distance: usize,
    tick_tolerance_when_member_arrives: usize,
    max_gathering_unit_groups: usize,
    max_unit_group_size: usize,
}

#[derive(Deserialize, Serialize)]
struct Default {
    radius: f32,
    separation_force: f32,
    separation_factor: f32,
    force_unit_fuzzy_goto_behavior: bool,
}

#[derive(Deserialize, Serialize)]
struct Moving {
    radius: usize,
    separation_force: f32,
    separation_factor: usize,
    force_unit_fuzzy_goto_behavior: bool,
}

#[derive(Deserialize, Serialize)]
struct Steering {
    default: Default,
    moving: Moving,
}

#[derive(Deserialize, Serialize)]
struct PathFinder {
    fwd2bwd_ratio: usize,
    goal_pressure_ratio: usize,
    max_steps_worked_per_tick: usize,
    max_work_done_per_tick: usize,
    use_path_cache: bool,
    short_cache_size: usize,
    long_cache_size: usize,
    short_cache_min_cacheable_distance: usize,
    short_cache_min_algo_steps_to_cache: usize,
    long_cache_min_cacheable_distance: usize,
    cache_max_connect_to_cache_steps_multiplier: usize,
    cache_accept_path_start_distance_ratio: f32,
    cache_accept_path_end_distance_ratio: f32,
    negative_cache_accept_path_start_distance_ratio: f32,
    negative_cache_accept_path_end_distance_ratio: f32,
    cache_path_start_distance_rating_multiplier: usize,
    cache_path_end_distance_rating_multiplier: usize,
    stale_enemy_with_same_destination_collision_penalty: usize,
    ignore_moving_enemy_collision_distance: usize,
    enemy_with_different_destination_collision_penalty: usize,
    general_entity_collision_penalty: usize,
    general_entity_subsequent_collision_penalty: usize,
    extended_collision_penalty: usize,
    max_clients_to_accept_any_new_request: usize,
    max_clients_to_accept_short_new_request: usize,
    direct_distance_to_consider_short_request: usize,
    short_request_max_steps: usize,
    short_request_ratio: f32,
    min_steps_to_check_path_find_termination: usize,
    start_to_goal_cost_multiplier_to_terminate_path_find: f32,
    overload_levels: Vec<usize>,
    overload_multipliers: Vec<usize>,
    negative_path_cache_delay_interval: usize,
}

#[derive(Deserialize, Serialize)]
struct MapSettings {
    difficulty_settings: DifficultySettings,
    pollution: Pollution,
    enemy_evolution: EnemyEvolution,
    enemy_expansion: EnemyExpansion,
    unit_group: UnitGroup,
    steering: Steering,
    path_finder: PathFinder,
    max_failed_behavior_count: usize,
}

#[derive(Deserialize, Serialize)]
struct AutoPlaceSetting {
    frequency: usize,
    size: usize,
    richness: usize,
}

#[derive(Deserialize, Serialize)]
struct AutoplaceControls {
    coal: AutoPlaceSetting,
    stone: AutoPlaceSetting,
    #[serde(rename(serialize = "copper-ore", deserialize = "copper-ore"))]
    copper_ore: AutoPlaceSetting,
    iron_ore: AutoPlaceSetting,
    #[serde(rename(serialize = "uranium-ore", deserialize = "uranium-ore"))]
    uranium_ore: AutoPlaceSetting,
    #[serde(rename(serialize = "crude-oil", deserialize = "crude-oil"))]
    crude_oil: AutoPlaceSetting,
    trees: AutoPlaceSetting,
    #[serde(rename(serialize = "enemy-base", deserialize = "enemy-base"))]
    enemy_base: AutoPlaceSetting,
}

#[derive(Deserialize, Serialize)]
struct CliffSettings {
    name: String,
    cliff_elevation_0: usize,
    cliff_elevation_interval: usize,
    richness: usize,
}

#[derive(Deserialize, Serialize)]
struct PropertyExpressionNames {
    #[serde(rename(
        serialize = "control-setting:moisture:frequency:multiplier",
        deserialize = "control-setting:moisture:frequency:multiplier"
    ))]
    control_setting_moisture_frequency_multiplier: String,
    #[serde(rename(
        serialize = "control-setting:moisture:bias",
        deserialize = "control-setting:moisture:bias"
    ))]
    control_setting_moisture_bias: String,
    #[serde(rename(
        serialize = "control-setting:aux:frequency:multiplier:",
        deserialize = "control-setting:aux:frequency:multiplier:"
    ))]
    control_setting_aux_frequency_multiplier: String,
    #[serde(rename(
        serialize = "control-setting:aux:bias",
        deserialize = "control-setting:aux:bias"
    ))]
    control_setting_aux_bias: String,
}

#[derive(Deserialize, Serialize)]
struct StartingPoint {
    x: usize,
    y: usize,
}

#[derive(Deserialize, Serialize)]
struct MapGenSettings {
    terrain_segmentation: usize,
    water: usize,
    width: usize,
    height: usize,
    starting_area: usize,
    peaceful_mode: bool,
    autoplace_controls: AutoplaceControls,
    cliff_settings: CliffSettings,
    property_expression_names: PropertyExpressionNames,
    starting_points: Vec<StartingPoint>,
    seed: Option<usize>,
}

extern crate proc_macro;

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() {
    Command::new("program").output();
}

async fn create_new_save() -> Result<(), ()> {
    let foo = Command::new("factorio")
        .arg("--create")
        .arg("save_data")
        .status()
        .await
        .or(Err(()))?;
    if foo.success() {
        Ok(())
    } else {
        Err(())
    }
}
