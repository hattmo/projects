use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct MapSettings {
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
struct Steering {
    default: SteeringSetting,
    moving: SteeringSetting,
}

#[derive(Deserialize, Serialize)]
struct SteeringSetting {
    radius: f32,
    separation_force: f32,
    separation_factor: f32,
    force_unit_fuzzy_goto_behavior: bool,
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

impl Default for MapSettings {
    fn default() -> Self {
        Self {
            difficulty_settings: DifficultySettings {
                recipe_difficulty: 0,
                technology_difficulty: 0,
                technology_price_multiplier: 1,
                research_queue_setting: "after-victory".to_string(),
            },
            pollution: Pollution {
                enabled: true,
                diffusion_ratio: 0.02,
                min_to_diffuse: 15,
                ageing: 1,
                expected_max_per_chunk: 150,
                min_to_show_per_chunk: 50,
                min_pollution_to_damage_trees: 60,
                pollution_with_max_forest_damage: 150,
                pollution_per_tree_damage: 50,
                pollution_restored_per_tree_damage: 10,
                max_pollution_to_restore_trees: 20,
                enemy_attack_pollution_consumption_modifier: 1,
            },
            enemy_evolution: EnemyEvolution {
                enabled: true,
                time_factor: 0.000004,
                destroy_factor: 0.002,
                pollution_factor: 0.0000009,
            },
            enemy_expansion: EnemyExpansion {
                enabled: true,
                min_base_spacing: 3,
                max_expansion_distance: 7,
                friendly_base_influence_radius: 2,
                enemy_building_influence_radius: 2,
                building_coefficient: 0.1,
                other_base_coefficient: 2.0,
                neighbouring_chunk_coefficient: 0.5,
                neighbouring_base_chunk_coefficient: 0.4,
                max_colliding_tiles_coefficient: 0.9,
                settler_group_min_size: 5,
                settler_group_max_size: 20,
                min_expansion_cooldown: 14400,
                max_expansion_cooldown: 216000,
            },
            unit_group: UnitGroup {
                min_group_gathering_time: 3600,
                max_group_gathering_time: 36000,
                max_wait_time_for_late_members: 7200,
                max_group_radius: 30.0,
                min_group_radius: 5.0,
                max_member_speedup_when_behind: 1.4,
                max_member_slowdown_when_ahead: 0.6,
                max_group_slowdown_factor: 0.3,
                max_group_member_fallback_factor: 3,
                member_disown_distance: 10,
                tick_tolerance_when_member_arrives: 60,
                max_gathering_unit_groups: 30,
                max_unit_group_size: 200,
            },
            steering: Steering {
                default: SteeringSetting {
                    radius: 1.2,
                    separation_force: 0.005,
                    separation_factor: 1.2,
                    force_unit_fuzzy_goto_behavior: false,
                },
                moving: SteeringSetting {
                    radius: 3.0,
                    separation_force: 0.01,
                    separation_factor: 3.0,
                    force_unit_fuzzy_goto_behavior: false,
                },
            },
            path_finder: PathFinder {
                fwd2bwd_ratio: 5,
                goal_pressure_ratio: 2,
                max_steps_worked_per_tick: 100,
                max_work_done_per_tick: 8000,
                use_path_cache: true,
                short_cache_size: 5,
                long_cache_size: 25,
                short_cache_min_cacheable_distance: 10,
                short_cache_min_algo_steps_to_cache: 50,
                long_cache_min_cacheable_distance: 30,
                cache_max_connect_to_cache_steps_multiplier: 100,
                cache_accept_path_start_distance_ratio: 0.2,
                cache_accept_path_end_distance_ratio: 0.15,
                negative_cache_accept_path_start_distance_ratio: 0.3,
                negative_cache_accept_path_end_distance_ratio: 0.3,
                cache_path_start_distance_rating_multiplier: 10,
                cache_path_end_distance_rating_multiplier: 20,
                stale_enemy_with_same_destination_collision_penalty: 30,
                ignore_moving_enemy_collision_distance: 5,
                enemy_with_different_destination_collision_penalty: 30,
                general_entity_collision_penalty: 10,
                general_entity_subsequent_collision_penalty: 3,
                extended_collision_penalty: 3,
                max_clients_to_accept_any_new_request: 10,
                max_clients_to_accept_short_new_request: 100,
                direct_distance_to_consider_short_request: 100,
                short_request_max_steps: 1000,
                short_request_ratio: 0.5,
                min_steps_to_check_path_find_termination: 2000,
                start_to_goal_cost_multiplier_to_terminate_path_find: 500.0,
                overload_levels: vec![0, 100, 500],
                overload_multipliers: vec![2, 3, 4],
                negative_path_cache_delay_interval: 20,
            },
            max_failed_behavior_count: 3,
        }
    }
}
