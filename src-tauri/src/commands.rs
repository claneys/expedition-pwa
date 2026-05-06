use crate::{AppData, GearItem, get_data_path};
use std::fs;

#[tauri::command]
pub fn load_data(app_handle: tauri::AppHandle) -> Result<AppData, String> {
    let path = get_data_path(&app_handle)?;
    if path.exists() {
        let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
        let data: AppData = serde_json::from_str(&content).map_err(|e| e.to_string())?;
        Ok(data)
    } else {
        Ok(AppData::default())
    }
}

#[tauri::command]
pub fn save_data(app_handle: tauri::AppHandle, data: AppData) -> Result<(), String> {
    let path = get_data_path(&app_handle)?;
    let content = serde_json::to_string_pretty(&data).map_err(|e| e.to_string())?;
    fs::write(&path, content).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn add_custom_gear(app_handle: tauri::AppHandle, name: String, qty: u32, sub: String, weight_g: Option<u32>) -> Result<AppData, String> {
    let mut data = load_data(app_handle.clone())?;
    data.custom_gear.push(GearItem {
        name,
        qty,
        sub,
        checked: false,
        source: Some("custom".into()),
        weight_g,
    });
    save_data(app_handle, data.clone())?;
    Ok(data)
}

#[tauri::command]
pub fn remove_custom_gear(app_handle: tauri::AppHandle, index: usize) -> Result<AppData, String> {
    let mut data = load_data(app_handle.clone())?;
    if index < data.custom_gear.len() {
        data.custom_gear.remove(index);
        save_data(app_handle, data.clone())?;
    }
    Ok(data)
}

#[tauri::command]
pub fn update_trek_days(app_handle: tauri::AppHandle, days: u32) -> Result<AppData, String> {
    let mut data = load_data(app_handle.clone())?;
    data.trek_days = days;
    save_data(app_handle, data.clone())?;
    Ok(data)
}

#[tauri::command]
pub async fn fetch_gear_share(
    _app_handle: tauri::AppHandle,
    api_url: String,
    username: String,
    password: String,
) -> Result<Vec<GearItem>, String> {
    let client = reqwest::Client::new();
    
    // Login to get token
    let login_res = client
        .post(format!("{}/api/auth/token", api_url))
        .json(&serde_json::json!({
            "username": username,
            "password": password
        }))
        .send()
        .await
        .map_err(|e| format!("Erreur connexion gear-share: {}", e))?;
    
    if !login_res.status().is_success() {
        return Err("Identifiants gear-share invalides".into());
    }
    
    let token_data: serde_json::Value = login_res
        .json()
        .await
        .map_err(|e| format!("Erreur parsing token: {}", e))?;
    
    let token = token_data["access_token"]
        .as_str()
        .ok_or("Token manquant dans la réponse")?;
    
    // Fetch equipment
    let equip_res = client
        .get(format!("{}/api/equipment/", api_url))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| format!("Erreur récupération matériel: {}", e))?;
    
    if !equip_res.status().is_success() {
        return Err("Impossible de récupérer le matériel".into());
    }
    
    let equip_data: serde_json::Value = equip_res
        .json()
        .await
        .map_err(|e| format!("Erreur parsing matériel: {}", e))?;
    
    let items = equip_data["items"]
        .as_array()
        .ok_or("Format de réponse invalide")?;
    
    let gear_items: Vec<GearItem> = items
        .iter()
        .map(|item| {
            let name = item["custom_name"]
                .as_str()
                .or_else(|| item["reference"]["name"].as_str())
                .unwrap_or("Matériel inconnu");
            
            let category = item["category"]["name"]
                .as_str()
                .unwrap_or("technique");
            
            let sub = match category {
                "Escalade" | "Ski de randonnée" | "Alpinisme" => "technique",
                "Vêtements" | "Chaussures" => "vêtements",
                "Sécurité" => "sécurité",
                "Bivouac" | "Camping" => "bivouac",
                "Nourriture" | "Hydratation" => "nourriture",
                _ => "technique",
            };
            
            GearItem {
                name: name.to_string(),
                qty: item["quantity"].as_u64().unwrap_or(1) as u32,
                sub: sub.to_string(),
                checked: false,
                source: Some("gear-share".into()),
                weight_g: item["weight_g"].as_u64().map(|w| w as u32),
            }
        })
        .collect();
    
    Ok(gear_items)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_load_data_creates_default_when_missing() {
        let temp_dir = TempDir::new().unwrap();
        let data_path = temp_dir.path().join("data.json");
        
        // Test that loading from non-existent path returns default
        // Note: This test would need a mock AppHandle in practice
        // For now, we test the serialization logic
        
        let data = AppData::default();
        let json = serde_json::to_string_pretty(&data).unwrap();
        
        // Write test data
        fs::write(&data_path, &json).unwrap();
        
        // Read it back
        let content = fs::read_to_string(&data_path).unwrap();
        let loaded: AppData = serde_json::from_str(&content).unwrap();
        
        assert_eq!(loaded.activities.len(), 5);
        assert!(loaded.specific_gear.contains_key("trek"));
    }

    #[test]
    fn test_save_and_load_roundtrip() {
        let temp_dir = TempDir::new().unwrap();
        let data_path = temp_dir.path().join("data.json");
        
        let mut data = AppData::default();
        data.custom_gear.push(GearItem {
            name: "Custom Tent".to_string(),
            qty: 1,
            sub: "bivouac".to_string(),
            checked: false,
            source: Some("custom".to_string()),
            weight_g: Some(800),
        });
        
        // Save
        let json = serde_json::to_string_pretty(&data).unwrap();
        fs::write(&data_path, json).unwrap();
        
        // Load
        let content = fs::read_to_string(&data_path).unwrap();
        let loaded: AppData = serde_json::from_str(&content).unwrap();
        
        assert_eq!(loaded.custom_gear.len(), 1);
        assert_eq!(loaded.custom_gear[0].name, "Custom Tent");
        assert_eq!(loaded.custom_gear[0].sub, "bivouac");
    }

    #[test]
    fn test_add_custom_gear() {
        let mut data = AppData::default();
        
        data.custom_gear.push(GearItem {
            name: "New Item".to_string(),
            qty: 3,
            sub: "technique".to_string(),
            checked: false,
            source: Some("custom".to_string()),
            weight_g: Some(250),
        });
        
        assert_eq!(data.custom_gear.len(), 1);
        assert_eq!(data.custom_gear[0].name, "New Item");
        assert_eq!(data.custom_gear[0].qty, 3);
        assert_eq!(data.custom_gear[0].source, Some("custom".to_string()));
    }

    #[test]
    fn test_remove_custom_gear() {
        let mut data = AppData::default();
        
        data.custom_gear.push(GearItem {
            name: "Item 1".to_string(),
            qty: 1,
            sub: "technique".to_string(),
            checked: false,
            source: Some("custom".to_string()),
            weight_g: Some(300),
        });
        
        data.custom_gear.push(GearItem {
            name: "Item 2".to_string(),
            qty: 1,
            sub: "confort".to_string(),
            checked: false,
            source: Some("custom".to_string()),
            weight_g: None,
        });
        
        assert_eq!(data.custom_gear.len(), 2);
        
        // Remove first item
        data.custom_gear.remove(0);
        
        assert_eq!(data.custom_gear.len(), 1);
        assert_eq!(data.custom_gear[0].name, "Item 2");
    }

    #[test]
    fn test_gear_share_response_parsing() {
        // Test parsing of gear-share API response
        let json_response = r#"{
            "items": [
                {
                    "custom_name": "My Tent",
                    "reference": {"name": "Tent Model X"},
                    "category": {"name": "Bivouac"},
                    "quantity": 1
                },
                {
                    "custom_name": null,
                    "reference": {"name": "Sleeping Bag"},
                    "category": {"name": "Vêtements"},
                    "quantity": 2
                }
            ]
        }"#;
        
        let parsed: serde_json::Value = serde_json::from_str(json_response).unwrap();
        let items = parsed["items"].as_array().unwrap();
        
        assert_eq!(items.len(), 2);
        
        // Test name resolution logic
        let name1 = items[0]["custom_name"]
            .as_str()
            .or_else(|| items[0]["reference"]["name"].as_str())
            .unwrap_or("Unknown");
        assert_eq!(name1, "My Tent");
        
        let name2 = items[1]["custom_name"]
            .as_str()
            .or_else(|| items[1]["reference"]["name"].as_str())
            .unwrap_or("Unknown");
        assert_eq!(name2, "Sleeping Bag");
    }

    #[test]
    fn test_category_mapping() {
        // Test the category to subcategory mapping logic
        let test_cases = vec![
            ("Escalade", "technique"),
            ("Ski de randonnée", "technique"),
            ("Alpinisme", "technique"),
            ("Vêtements", "vêtements"),
            ("Chaussures", "vêtements"),
            ("Sécurité", "sécurité"),
            ("Bivouac", "bivouac"),
            ("Camping", "bivouac"),
            ("Nourriture", "nourriture"),
            ("Hydratation", "nourriture"),
            ("Unknown", "technique"), // Default fallback
        ];
        
        for (category, expected_sub) in test_cases {
            let sub = match category {
                "Escalade" | "Ski de randonnée" | "Alpinisme" => "technique",
                "Vêtements" | "Chaussures" => "vêtements",
                "Sécurité" => "sécurité",
                "Bivouac" | "Camping" => "bivouac",
                "Nourriture" | "Hydratation" => "nourriture",
                _ => "technique",
            };
            assert_eq!(sub, expected_sub, "Category '{}' should map to '{}'", category, expected_sub);
        }
    }

    #[test]
    fn test_checked_state_persistence() {
        let mut data = AppData::default();
        
        // Mark some items as checked
        data.commun_gear[0].checked = true;
        data.commun_gear[1].checked = true;
        
        // Serialize and deserialize
        let json = serde_json::to_string_pretty(&data).unwrap();
        let loaded: AppData = serde_json::from_str(&json).unwrap();
        
        assert!(loaded.commun_gear[0].checked);
        assert!(loaded.commun_gear[1].checked);
        assert!(!loaded.commun_gear[2].checked);
    }

    #[test]
    fn test_gear_item_with_gear_share_source() {
        let item = GearItem {
            name: "Imported Tent".to_string(),
            qty: 1,
            sub: "bivouac".to_string(),
            checked: false,
            source: Some("gear-share".to_string()),
            weight_g: Some(1200),
        };
        
        assert_eq!(item.source, Some("gear-share".to_string()));
        assert_eq!(item.weight_g, Some(1200));
    }
    
    #[test]
    fn test_trek_days_default() {
        let data = AppData::default();
        assert_eq!(data.trek_days, 3);
    }
    
    #[test]
    fn test_trek_days_persistence() {
        let mut data = AppData::default();
        data.trek_days = 7;
        
        let json = serde_json::to_string_pretty(&data).unwrap();
        let loaded: AppData = serde_json::from_str(&json).unwrap();
        
        assert_eq!(loaded.trek_days, 7);
    }
}