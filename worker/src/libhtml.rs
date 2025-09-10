use serde::Deserialize;
use crate::models::{FinancialData, JsonPathConfig, CodeType};
use crate::utils::get_code_type;
use worker::kv::KvStore;
use serde_json::{Value, Map};
use std::collections::HashMap;

// --- Helper for dynamic JSON path traversal ---

/// Gets a value from a serde_json::Value using a path string (e.g., "/a/b/c").
/// Returns None if the path is invalid or value not found.
fn get_value_from_json_path<'a>(json_value: &'a Value, path: &str) -> Option<&'a Value> {
    let mut current_value = json_value;
    for part in path.trim_start_matches('/').split('/') {
        if part.is_empty() { continue; }
        current_value = current_value.get(part)?;
    }
    Some(current_value)
}

/// Recursively searches for a value in a JSON object/array based on a keyword.
/// Returns the path to the first found value.
fn find_path_to_value(json_value: &Value, keyword: &str, current_path: &mut String) -> Option<String> {
    match json_value {
        Value::Object(map) => {
            for (key, value) in map {
                let original_len = current_path.len();
                current_path.push_str(&format!("/{}", key));

                // Check if the key itself contains the keyword (case-insensitive)
                if key.to_lowercase().contains(&keyword.to_lowercase()) {
                    // Also ensure the value is a primitive, not another object/array
                    if !value.is_object() && !value.is_array() {
                        return Some(current_path.clone());
                    }
                }

                // Check if the value is a string and contains the keyword
                if let Some(s) = value.as_str() {
                    if s.to_lowercase().contains(&keyword.to_lowercase()) {
                        return Some(current_path.clone());
                    }
                }

                // Recursively search in nested objects/arrays
                if let Some(found_path) = find_path_to_value(value, keyword, current_path) {
                    return Some(found_path);
                }
                current_path.truncate(original_len);
            }
        }
        Value::Array(array) => {
            for (i, value) in array.iter().enumerate() {
                let original_len = current_path.len();
                current_path.push_str(&format!("/{}", i));
                if let Some(found_path) = find_path_to_value(value, keyword, current_path) {
                    return Some(found_path);
                }
                current_path.truncate(original_len);
            }
        }
        _ => {}
    }
    None
}

// --- Main Parsing Function (Self-Healing) ---

/// Parses financial data from the __PRELOADED_STATE__ JSON embedded in HTML.
/// Now includes self-healing for JSON structure changes.
pub async fn parse_from_preloaded_state(html_content: &str, code: &str, kv: &KvStore) -> Result<FinancialData, String> {
    const SCRIPT_START: &str = "window.__PRELOADED_STATE__ = ";
    let script_line = html_content
        .lines()
        .find(|line| line.trim().starts_with(SCRIPT_START));

    let json_str = if let Some(line) = script_line {
        line.trim()
            .strip_prefix(SCRIPT_START)
            .and_then(|s| s.strip_suffix(';'))
            .unwrap_or_else(|| &line.trim()[SCRIPT_START.len()..])
    } else {
        return Err("Could not find __PRELOADED_STATE__ in HTML content".to_string());
    };

    let root_json: Value = serde_json::from_str(json_str)
        .map_err(|e| format!("JSON parsing error: {}", e))?;

    let mut data = FinancialData {
        code: Some(code.to_string()),
        ..Default::default()
    };

    // Determine code type to select appropriate JSON paths
    let code_type = get_code_type(code);

    // Define default paths for stock and index
    let stock_paths = HashMap::from([
        ("name", "/mainStocksPriceBoard/priceBoard/name"),
        ("price", "/mainStocksPriceBoard/priceBoard/price"),
        ("priceChange", "/mainStocksPriceBoard/priceBoard/priceChange"),
        ("priceChangeRate", "/mainStocksPriceBoard/priceBoard/priceChangeRate"),
        ("priceDateTime", "/mainStocksPriceBoard/priceBoard/priceDateTime"),
    ]);

    let nikkei_paths = HashMap::from([
        ("name", "/mainDomesticIndexPriceBoard/indexPrices/name"),
        ("price", "/mainDomesticIndexPriceBoard/indexPrices/price"),
        ("priceChange", "/mainDomesticIndexPriceBoard/indexPrices/changePrice"),
        ("priceChangeRate", "/mainDomesticIndexPriceBoard/indexPrices/changePriceRate"),
        ("priceDateTime", "/mainDomesticIndexPriceBoard/indexPrices/japanUpdateTime"),
    ]);

    let default_paths = match code_type {
        CodeType::Stock => &stock_paths,
        CodeType::Nikkei => &nikkei_paths,
        _ => &stock_paths, // Default to stock paths for other types or unknown
    };

    // Try to load existing paths from KV
    let mut json_paths: JsonPathConfig = kv.get(&format!("json_paths_{}", code))
        .json().await.unwrap_or_default().unwrap_or_default();

    let mut updated_paths = false;

    // Iterate through fields, try to get value by path, or find new path
    let mut fields_to_find: HashMap<&str, &mut Option<String>> = HashMap::new();
    fields_to_find.insert("name", &mut data.name);
    fields_to_find.insert("price", &mut data.current_value);
    fields_to_find.insert("priceChange", &mut data.previous_day_change);
    fields_to_find.insert("priceChangeRate", &mut data.change_rate);
    fields_to_find.insert("priceDateTime", &mut data.update_time);

    for (field_name, data_field) in fields_to_find.iter_mut() {
        let kv_path = match *field_name {
            "name" => &json_paths.name_path,
            "price" => &json_paths.current_value_path,
            "priceChange" => &json_paths.previous_day_change_path,
            "priceChangeRate" => &json_paths.change_rate_path,
            "priceDateTime" => &json_paths.update_time_path,
            _ => &None, // Should not happen
        };

        let mut found_value = None;
        let mut new_path_found = None;

        // 1. Try to get value using existing path from KV
        if let Some(path) = kv_path {
            if let Some(value) = get_value_from_json_path(&root_json, path) {
                if !value.is_object() && !value.is_array() {
                    found_value = Some(value.to_string().trim_matches('"').to_string());
                }
            }
        }

        // 2. If value not found or KV path is empty, try default path based on code type
        if found_value.is_none() { // Removed `kv_path.is_none()` as we always try default if KV fails
            if let Some(&default_path) = default_paths.get(field_name) {
                if let Some(value) = get_value_from_json_path(&root_json, default_path) {
                    if !value.is_object() && !value.is_array() {
                        found_value = Some(value.to_string().trim_matches('"').to_string());
                        new_path_found = Some(default_path.to_string()); // Save the new default path
                        updated_paths = true;
                    }
                }
            }
        }

        // Update FinancialData and JsonPathConfig
        if let Some(val) = found_value {
            **data_field = Some(val);
        }
        if let Some(path) = new_path_found {
            match *field_name {
                "name" => json_paths.name_path = Some(path),
                "price" => json_paths.current_value_path = Some(path),
                "priceChange" => json_paths.previous_day_change_path = Some(path),
                "priceChangeRate" => json_paths.change_rate_path = Some(path),
                "priceDateTime" => json_paths.update_time_path = Some(path),
                _ => {}, // Should not happen
            }
        }
    }

    // Save updated paths to KV if any were found/updated
    if updated_paths {
        kv.put(&format!("json_paths_{}", code), json_paths)
            .map_err(|e| e.to_string())?
            .execute()
            .await
            .map_err(|e| format!("Failed to save JSON paths to KV: {}", e))?;
        worker::console_log!("Saved updated JSON paths for {}: {:?}", code, kv.get(&format!("json_paths_{}", code)).json::<JsonPathConfig>().await);
    }

    Ok(data)
}