use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fs;
use tauri::Manager;

pub mod commands;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GearItem {
    pub name: String,
    pub qty: u32,
    pub sub: String,
    pub checked: bool,
    pub source: Option<String>, // "default", "gear-share", "custom"
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Activity {
    pub id: String,
    pub name: String,
    pub icon: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppData {
    pub activities: Vec<Activity>,
    pub specific_gear: std::collections::HashMap<String, Vec<GearItem>>,
    pub commun_gear: Vec<GearItem>,
    pub custom_gear: Vec<GearItem>,       // Items ajoutés par l'utilisateur
    pub gear_share_config: Option<GearShareConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GearShareConfig {
    pub api_url: String,
    pub username: String,
    pub token: String,
}

impl Default for AppData {
    fn default() -> Self {
        AppData {
            activities: vec![
                Activity { id: "ski-rando".into(), name: "Ski de rando".into(), icon: "⛷️".into() },
                Activity { id: "alpinisme".into(), name: "Alpinisme".into(), icon: "🧗".into() },
                Activity { id: "escalade".into(), name: "Escalade".into(), icon: "🧗".into() },
                Activity { id: "randonnee".into(), name: "Randonnée".into(), icon: "🥾".into() },
                Activity { id: "trek".into(), name: "Trek".into(), icon: "🏕️".into() },
            ],
            specific_gear: [
                ("ski-rando".to_string(), vec![
                    GearItem { name: "Skis".into(), qty: 1, sub: "technique".into(), checked: false, source: Some("default".into()) },
                    GearItem { name: "Peaux".into(), qty: 1, sub: "technique".into(), checked: false, source: Some("default".into()) },
                    GearItem { name: "Chaussures de skis".into(), qty: 1, sub: "technique".into(), checked: false, source: Some("default".into()) },
                    GearItem { name: "Bâtons".into(), qty: 1, sub: "technique".into(), checked: false, source: Some("default".into()) },
                    GearItem { name: "Couteaux".into(), qty: 1, sub: "technique".into(), checked: false, source: Some("default".into()) },
                ]),
                ("alpinisme".to_string(), vec![
                    GearItem { name: "Crampons".into(), qty: 1, sub: "technique".into(), checked: false, source: Some("default".into()) },
                    GearItem { name: "Piolet".into(), qty: 1, sub: "technique".into(), checked: false, source: Some("default".into()) },
                    GearItem { name: "Casque".into(), qty: 1, sub: "sécurité".into(), checked: false, source: Some("default".into()) },
                    GearItem { name: "Baudrier".into(), qty: 1, sub: "technique".into(), checked: false, source: Some("default".into()) },
                ]),
                ("escalade".to_string(), vec![
                    GearItem { name: "Corde simple".into(), qty: 1, sub: "technique".into(), checked: false, source: Some("default".into()) },
                    GearItem { name: "Corde double".into(), qty: 1, sub: "technique".into(), checked: false, source: Some("default".into()) },
                ]),
                ("randonnee".to_string(), vec![
                    GearItem { name: "Bâtons randonné".into(), qty: 1, sub: "technique".into(), checked: false, source: Some("default".into()) },
                ]),
                ("trek".to_string(), vec![
                    GearItem { name: "Tente / Tarp".into(), qty: 1, sub: "bivouac".into(), checked: false, source: Some("default".into()) },
                    GearItem { name: "Sac de couchage".into(), qty: 1, sub: "bivouac".into(), checked: false, source: Some("default".into()) },
                    GearItem { name: "Matelas".into(), qty: 1, sub: "bivouac".into(), checked: false, source: Some("default".into()) },
                    GearItem { name: "Réchaud + cartouche".into(), qty: 1, sub: "bivouac".into(), checked: false, source: Some("default".into()) },
                    GearItem { name: "Popote + couverts".into(), qty: 1, sub: "bivouac".into(), checked: false, source: Some("default".into()) },
                    GearItem { name: "Nourriture 3j".into(), qty: 1, sub: "nourriture".into(), checked: false, source: Some("default".into()) },
                    GearItem { name: "Corde".into(), qty: 1, sub: "technique".into(), checked: false, source: Some("default".into()) },
                ]),
            ].into_iter().collect(),
            commun_gear: vec![
                GearItem { name: "Collant Mérinos".into(), qty: 1, sub: "vêtements".into(), checked: false, source: Some("default".into()) },
                GearItem { name: "Pantalon ski".into(), qty: 1, sub: "vêtements".into(), checked: false, source: Some("default".into()) },
                GearItem { name: "Chaussettes ski".into(), qty: 1, sub: "vêtements".into(), checked: false, source: Some("default".into()) },
                GearItem { name: "Polaire".into(), qty: 1, sub: "vêtements".into(), checked: false, source: Some("default".into()) },
                GearItem { name: "Goretex".into(), qty: 1, sub: "vêtements".into(), checked: false, source: Some("default".into()) },
                GearItem { name: "Doudoune".into(), qty: 1, sub: "vêtements".into(), checked: false, source: Some("default".into()) },
                GearItem { name: "Bonnet-buff".into(), qty: 1, sub: "vêtements".into(), checked: false, source: Some("default".into()) },
                GearItem { name: "Gants".into(), qty: 2, sub: "vêtements".into(), checked: false, source: Some("default".into()) },
                GearItem { name: "Lunettes soleil".into(), qty: 1, sub: "vêtements".into(), checked: false, source: Some("default".into()) },
                GearItem { name: "Masque".into(), qty: 1, sub: "vêtements".into(), checked: false, source: Some("default".into()) },
                GearItem { name: "Crème solaire".into(), qty: 1, sub: "vêtements".into(), checked: false, source: Some("default".into()) },
                GearItem { name: "Frontale".into(), qty: 1, sub: "sécurité".into(), checked: false, source: Some("default".into()) },
                GearItem { name: "Sac à viande".into(), qty: 1, sub: "sécurité".into(), checked: false, source: Some("default".into()) },
                GearItem { name: "Médicaments + boules quies".into(), qty: 1, sub: "sécurité".into(), checked: false, source: Some("default".into()) },
                GearItem { name: "Gourde".into(), qty: 1, sub: "confort".into(), checked: false, source: Some("default".into()) },
                GearItem { name: "Thermos".into(), qty: 1, sub: "confort".into(), checked: false, source: Some("default".into()) },
                GearItem { name: "Barres énergétiques".into(), qty: 1, sub: "confort".into(), checked: false, source: Some("default".into()) },
                GearItem { name: "Chargeur téléphone".into(), qty: 1, sub: "confort".into(), checked: false, source: Some("default".into()) },
                GearItem { name: "Batterie externe".into(), qty: 1, sub: "confort".into(), checked: false, source: Some("default".into()) },
                GearItem { name: "Musique / Écouteurs".into(), qty: 1, sub: "confort".into(), checked: false, source: Some("default".into()) },
                GearItem { name: "Microserviette + nécessaire toilette".into(), qty: 1, sub: "confort".into(), checked: false, source: Some("default".into()) },
                GearItem { name: "Sac à dos 35L mini".into(), qty: 1, sub: "technique".into(), checked: false, source: Some("default".into()) },
                GearItem { name: "Cash (ouèche pour refuge)".into(), qty: 1, sub: "technique".into(), checked: false, source: Some("default".into()) },
                GearItem { name: "Collant + sous-pull rechange refuge".into(), qty: 1, sub: "technique".into(), checked: false, source: Some("default".into()) },
            ],
            custom_gear: vec![],
            gear_share_config: None,
        }
    }
}

pub fn get_data_path(app_handle: &tauri::AppHandle) -> Result<PathBuf, String> {
    let app_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?;
    fs::create_dir_all(&app_dir).map_err(|e| e.to_string())?;
    Ok(app_dir.join("data.json"))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::load_data,
            commands::save_data,
            commands::add_custom_gear,
            commands::remove_custom_gear,
            commands::fetch_gear_share
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gear_item_creation() {
        let item = GearItem {
            name: "Test Item".to_string(),
            qty: 2,
            sub: "technique".to_string(),
            checked: false,
            source: Some("default".to_string()),
        };
        
        assert_eq!(item.name, "Test Item");
        assert_eq!(item.qty, 2);
        assert_eq!(item.sub, "technique");
        assert!(!item.checked);
        assert_eq!(item.source, Some("default".to_string()));
    }

    #[test]
    fn test_activity_creation() {
        let activity = Activity {
            id: "test".to_string(),
            name: "Test Activity".to_string(),
            icon: "🧪".to_string(),
        };
        
        assert_eq!(activity.id, "test");
        assert_eq!(activity.name, "Test Activity");
        assert_eq!(activity.icon, "🧪");
    }

    #[test]
    fn test_app_data_default() {
        let data = AppData::default();
        
        // Check activities
        assert_eq!(data.activities.len(), 5);
        assert_eq!(data.activities[0].id, "ski-rando");
        assert_eq!(data.activities[4].id, "trek");
        
        // Check specific gear
        assert!(data.specific_gear.contains_key("ski-rando"));
        assert!(data.specific_gear.contains_key("trek"));
        assert_eq!(data.specific_gear["ski-rando"].len(), 5);
        assert_eq!(data.specific_gear["trek"].len(), 7); // Includes bivouac items
        
        // Check commun gear
        assert!(!data.commun_gear.is_empty());
        
        // Check custom gear is empty by default
        assert!(data.custom_gear.is_empty());
        
        // Check gear share config is None by default
        assert!(data.gear_share_config.is_none());
    }

    #[test]
    fn test_app_data_serialization() {
        let data = AppData::default();
        let json = serde_json::to_string(&data).expect("Failed to serialize");
        let deserialized: AppData = serde_json::from_str(&json).expect("Failed to deserialize");
        
        assert_eq!(data.activities.len(), deserialized.activities.len());
        assert_eq!(data.commun_gear.len(), deserialized.commun_gear.len());
    }

    #[test]
    fn test_gear_share_config() {
        let config = GearShareConfig {
            api_url: "https://gear-share.example.com".to_string(),
            username: "test_user".to_string(),
            token: "test_token".to_string(),
        };
        
        assert_eq!(config.api_url, "https://gear-share.example.com");
        assert_eq!(config.username, "test_user");
        assert_eq!(config.token, "test_token");
    }

    #[test]
    fn test_gear_item_categories() {
        let data = AppData::default();
        
        // Test that trek activity has bivouac items
        let trek_gear = data.specific_gear.get("trek").expect("Trek gear should exist");
        let bivouac_items: Vec<_> = trek_gear.iter().filter(|g| g.sub == "bivouac").collect();
        assert!(!bivouac_items.is_empty(), "Trek should have bivouac items");
        
        // Test that trek activity has nourriture items
        let nourriture_items: Vec<_> = trek_gear.iter().filter(|g| g.sub == "nourriture").collect();
        assert!(!nourriture_items.is_empty(), "Trek should have nourriture items");
    }

    #[test]
    fn test_all_gear_items_have_source() {
        let data = AppData::default();
        
        // Check all default items have source set
        for (_, items) in &data.specific_gear {
            for item in items {
                assert_eq!(item.source, Some("default".to_string()), 
                    "Item {} should have source 'default'", item.name);
            }
        }
        
        for item in &data.commun_gear {
            assert_eq!(item.source, Some("default".to_string()),
                "Item {} should have source 'default'", item.name);
        }
    }
}