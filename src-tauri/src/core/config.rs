use tauri::{AppHandle, Emitter, State};
use crate::composent::config::BotConfig;
use crate::models::{AppState, RegionCoordinates};



#[tauri::command]
pub async fn call_get_config(state: State<'_, AppState>) -> Result<BotConfig, String> {
    let state = state.inner.lock().unwrap();
    let config = state.config.clone();




    Ok(config)
}

#[tauri::command]
pub async fn call_update_config(
    new_config: BotConfig,
    state: State<'_, AppState>,
    app: AppHandle
) -> Result<(), String> {
    let mut app_state = state.inner.lock().unwrap();
    app_state.config = new_config;
    app_state.config.save().map_err(|e| e.to_string())?;


    app.emit("state_changed", &*app_state).unwrap();
    Ok(())
}




#[tauri::command]
pub async fn call_save_region(
    region_data: RegionCoordinates,
    state: State<'_, AppState>, // Supposons que `AppState` contient votre configuration
    app: AppHandle
) -> Result<(), String> {
    let region = region_data.region;
    let coordinates = region_data.coordinates;

    let mut state = state.inner.lock().unwrap();

    // Convertir [i32; 4] en (i32, i32, u32, u32)
    let (x1, y1, x2, y2) = (
        coordinates[0],        // x1
        coordinates[1],        // y1
        coordinates[2] as u32, // x2 (convertir en u32)
        coordinates[3] as u32, // y2 (convertir en u32)
    );

    // Assigner les coordonnées à la région correspondante
    match region.as_str() {
        "coordinates" => state.config.regions.coordinates = (x1, y1, x2, y2),
        "hunt_panel" => state.config.regions.hunt_panel = (x1, y1, x2, y2),
        "chat" => state.config.regions.chat = (x1, y1, x2, y2),
        _ => return Err(format!("Région inconnue : {}", region)),
    }

    state.config.save().map_err(|e| e.to_string())?;

    app.emit("state_changed", &*state).unwrap();

    println!(
        "Sauvegarde de la région '{}' avec les coordonnées : {:?}",
        region,
        (x1, y1, x2, y2)
    );

    Ok(())
}