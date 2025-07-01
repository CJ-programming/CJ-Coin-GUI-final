use reqwest::blocking::get;
use serde_json::Value;
/*
#[tauri::command]
pub fn discover_nodes(seed_ip: &str, seed_port: u16) -> Result<Value, String> {
    let url = format!("http://{}:{}/discover/nodes", seed_ip, seed_port);

    match get(&url) {
        Ok(response) => {
            match response.json::<Value>() {
                Ok(json) => Ok(json),
                Err(err) => Err(format!("JSON parse error: {}", err)),
            }
        }
        Err(err) => Err(format!("Request failed: {}", err)),
    }
}

use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct NodeNetAddr {
    pub ipv4_address: String,
    pub port: u16,
}

#[tauri::command]
pub fn broadcast_nodes_block(block: Value, nodes: Vec<NodeNetAddr>) -> Result<Value, String> {
    let client = Client::new();
    let mut responses: Vec<String> = Vec::new();

    for node in &nodes {
        let url = format!("http://{}:{}/validate/block", node.ipv4_address, node.port);
        match client.post(&url).json(&block).send() {
            Ok(resp) => match resp.json::<Value>() {
                Ok(json_resp) => responses.push(json_resp.to_string()),
                Err(err) => return Err(format!("Error parsing JSON response: {}", err)),
            },
            Err(err) => return Err(format!("Error posting to node: {}", err)),
        }
    }

    // Mode logic (simplified)
    let mut frequency: HashMap<String, usize> = HashMap::new();
    for r in &responses {
        *frequency.entry(r.clone()).or_insert(0) += 1;
    }

    let most_common = frequency
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .ok_or("No valid responses from nodes")?;

    serde_json::from_str(&most_common).map_err(|e| format!("Failed to parse most common response: {}", e))
}

*/