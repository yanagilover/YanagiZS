use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigInteractScale {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Deserialize)]
#[serde(tag = "$type")]
pub enum ConfigEventAction {
    #[serde(rename = "Share.CActionCreateNPCCfg")]
    ActionCreateNpcCfg {
        #[serde(rename = "ID")]
        id: i32,
        #[serde(rename = "TagID")]
        tag_id: i32,
    },
    #[serde(rename = "Share.CActionChangeInteractCfg")]
    #[serde(rename_all = "PascalCase")]
    ActionChangeInteractCfg {
        #[serde(rename = "ID")]
        id: u32,
        #[serde(rename = "InteractID")]
        interact_id: i32,
        #[serde(rename = "TagIDs")]
        tag_ids: Vec<i32>,
        #[serde(deserialize_with = "deserialize_participators_map")]
        participators: HashMap<u32, String>,
        interact_shape: String,
        interact_scale: ConfigInteractScale,
        #[serde(default)]
        section_listen_events: HashMap<String, String>,
    },
    #[serde(rename = "Share.CActionSetMainCityObjectState")]
    ActionSetMainCityObjectState {
        #[serde(rename = "ID")]
        id: u32,
        #[serde(rename = "ObjectState")]
        #[serde(deserialize_with = "deserialize_i32_map")]
        object_state: HashMap<i32, i32>,
    },
    #[serde(rename = "Share.CActionSwitchSection")]
    #[serde(rename_all = "PascalCase")]
    ActionSwitchSection {
        #[serde(rename = "SectionID")]
        section_id: u32,
        transform: String,
        camera_x: u32,
        camera_y: u32,
    },
    #[serde(rename = "Share.CActionOpenUI")]
    #[serde(rename_all = "PascalCase")]
    ActionOpenUI {
        #[serde(rename = "UI")]
        ui: String,
        args: i32,
        #[serde(rename = "StoreTemplateID")]
        store_template_id: i32,
    },
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigEvent {
    #[serde(rename = "ID")]
    #[serde(default)]
    pub id: u32,
    pub actions: Vec<ConfigEventAction>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SectionEventGraphConfig {
    #[serde(rename = "ID")]
    pub id: u32,
    pub on_add: Vec<String>,
    pub on_enter: Vec<String>,
    pub events: HashMap<String, ConfigEvent>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MainCitySectionConfig {
    #[serde(rename = "ID")]
    pub id: i32,
    pub unity_scene_path: String,
    pub born_transform: String,
    pub section_progress: SectionEventGraphConfig,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MainCityConfig {
    #[serde(rename = "ID")]
    pub id: u32,
    #[serde(rename = "DefaultSectionID")]
    pub default_section_id: u32,
    pub sections: HashMap<i32, MainCitySectionConfig>,
}

fn deserialize_participators_map<'de, D>(deserializer: D) -> Result<HashMap<u32, String>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    use serde::de;
    let str_map = HashMap::<&str, String>::deserialize(deserializer)?;

    Ok(str_map
        .into_iter()
        .map(|(str_key, value)| match str_key.parse() {
            Ok(int_key) => Ok((int_key, value)),
            Err(_) => Err(de::Error::invalid_value(
                de::Unexpected::Str(str_key),
                &"u32",
            )),
        })
        .collect::<Result<HashMap<_, _>, _>>()?)
}

fn deserialize_i32_map<'de, D>(deserializer: D) -> Result<HashMap<i32, i32>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    use serde::de;
    let str_map = HashMap::<&str, i32>::deserialize(deserializer)?;

    Ok(str_map
        .into_iter()
        .map(|(str_key, value)| match str_key.parse() {
            Ok(int_key) => Ok((int_key, value)),
            Err(_) => Err(de::Error::invalid_value(
                de::Unexpected::Str(str_key),
                &"i32",
            )),
        })
        .collect::<Result<HashMap<_, _>, _>>()?)
}
