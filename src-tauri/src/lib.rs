// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::Manager;
use window_vibrancy::apply_blur;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            if let Some(window) = app.get_webview_window("main") {
                // Position in top-right corner
                if let Some(monitor) = window.current_monitor()? {
                    let size = monitor.size();
                    let window_size = window.outer_size()?;
                    window.set_position(tauri::PhysicalPosition {
                        x: size.width - window_size.width,
                        y: 0,
                    })?;
                }

                // Apply vibrancy
                #[cfg(target_os = "windows")]
                apply_blur(&window, Some((18, 18, 18, 125)))?;

                #[cfg(target_os = "macos")]
                window_vibrancy::apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, None)?;
            }
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_app_handle, _event| {});
}
