// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Deserialize;

type CommandResult<T> = Result<T, String>;

#[derive(Deserialize, Debug)]
struct FmpApiResponse {
    historical: Option<Vec<HistoricalData>>,
    
    #[serde(rename = "Error Message")]
    error_message: Option<String>,
}

#[derive(Deserialize, Debug)]
struct HistoricalData {
    date: String,
}


#[tauri::command]
async fn validate_symbol(symbol: String, api_key: String) -> CommandResult<()> {
    let url = format!(
        "https://financialmodelingprep.com/api/v3/historical-price-full/{}?apikey={}",
        symbol, api_key
    );

    let response = reqwest::get(&url).await.map_err(|e| e.to_string())?;

    if response.status().is_success() {
        let api_data: FmpApiResponse = response.json().await.map_err(|e| e.to_string())?;

        if let Some(error) = api_data.error_message {
            return Err(format!("Error de la API: {}", error));
        }

        match api_data.historical {
            Some(data) if !data.is_empty() => Ok(()),
            _ => Err("Símbolo no encontrado o sin datos.".to_string()),
        }
    } else {
        Err(format!("Error de red: {}", response.status()))
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![validate_symbol])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}