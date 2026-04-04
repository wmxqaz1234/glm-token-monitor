use tauri::AppHandle;

#[cfg(target_os = "macos")]
#[allow(dead_code)]
pub fn create_system_tray(app: &AppHandle) -> Result<(), String> {
    println!("macOS system tray created");
    Ok(())
}

#[cfg(not(target_os = "macos"))]
#[allow(dead_code)]
pub fn create_system_tray(_app: &AppHandle) -> Result<(), String> {
    Ok(())
}
