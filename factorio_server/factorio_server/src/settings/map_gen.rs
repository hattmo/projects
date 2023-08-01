use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct MapGenSettings {
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

#[derive(Deserialize, Serialize)]
struct AutoplaceControls {
    coal: AutoPlaceSetting,
    stone: AutoPlaceSetting,
    #[serde(rename = "copper-ore")]
    copper_ore: AutoPlaceSetting,
    #[serde(rename = "iron-ore")]
    iron_ore: AutoPlaceSetting,
    #[serde(rename = "uranium-ore")]
    uranium_ore: AutoPlaceSetting,
    #[serde(rename = "crude-oil")]
    crude_oil: AutoPlaceSetting,
    trees: AutoPlaceSetting,
    #[serde(rename = "enemy-base")]
    enemy_base: AutoPlaceSetting,
}

#[derive(Deserialize, Serialize, Clone)]
struct AutoPlaceSetting {
    frequency: usize,
    size: usize,
    richness: usize,
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
    #[serde(rename = "control-setting:moisture:frequency:multiplier")]
    control_setting_moisture_frequency_multiplier: String,
    #[serde(rename = "control-setting:moisture:bias")]
    control_setting_moisture_bias: String,
    #[serde(rename = "control-setting:aux:frequency:multiplier:")]
    control_setting_aux_frequency_multiplier: String,
    #[serde(rename = "control-setting:aux:bias")]
    control_setting_aux_bias: String,
}

#[derive(Deserialize, Serialize)]
struct StartingPoint {
    x: usize,
    y: usize,
}

impl Default for MapGenSettings {
    fn default() -> Self {
        let auto_place = AutoPlaceSetting {
            frequency: 1,
            size: 1,
            richness: 1,
        };
        Self {
            terrain_segmentation: 1,
            water: 1,
            width: 0,
            height: 0,
            starting_area: 1,
            peaceful_mode: false,
            autoplace_controls: AutoplaceControls {
                coal: auto_place.clone(),
                stone: auto_place.clone(),
                copper_ore: auto_place.clone(),
                iron_ore: auto_place.clone(),
                uranium_ore: auto_place.clone(),
                crude_oil: auto_place.clone(),
                trees: auto_place.clone(),
                enemy_base: auto_place,
            },
            cliff_settings: CliffSettings {
                name: "cliff".to_string(),
                cliff_elevation_0: 10,
                cliff_elevation_interval: 40,
                richness: 1,
            },
            property_expression_names: PropertyExpressionNames {
                control_setting_moisture_frequency_multiplier: "1".to_string(),
                control_setting_moisture_bias: "0".to_string(),
                control_setting_aux_frequency_multiplier: "1".to_string(),
                control_setting_aux_bias: "0".to_string(),
            },
            starting_points: vec![StartingPoint { x: 0, y: 0 }],
            seed: None,
        }
    }
}
