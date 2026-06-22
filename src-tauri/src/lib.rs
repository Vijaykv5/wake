
// #[derive(serde::Serialise)]
// struct BatteryInfo{
//     percentage : f32,
//     charging :  bool
// }

// #[tauri::command] //expose this rust function to frontend so that that the react can use it 
// fn get_battery_info() -> Result<BatteryInfo, String>{
//     let manager = battery::Manager::new().map_err(|e| e.to_string())?;

//     let mut batteries = manager.batteries().map_err(|e| e.to_string())?;

//     if let Some(battery_result)=batteries.next(){
//         let battery = battery_result.map_err(|e| e.to_string())?;
//     }

// }







// #[cfg_attr(mobile, tauri::mobile_entry_point)]
// pub fn run() {
//     tauri::Builder::default()
//         .plugin(tauri_plugin_opener::init())
//         .invoke_handler(tauri::generate_handler![greet])
//         .run(tauri::generate_context!())
//         .expect("error while running tauri application");
// }
