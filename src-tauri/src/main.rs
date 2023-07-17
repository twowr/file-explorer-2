// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use sysinfo::{
    System, SystemExt,
    DiskExt
};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn get_mount_points() -> Vec<String> {
    let mut sys = System::new();
    sys.refresh_disks_list();
    let mount_points = sys.disks().iter().map(|disk| disk.mount_point().to_str());
    let mount_points:Vec<String> = mount_points.map(|mount_point_option|
        match mount_point_option {
            Some(mount_point) => mount_point.to_string(),
            None => {"".to_string()}
        }
    ).filter(|mount_point| mount_point.len() > 0).collect();

    mount_points
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_mount_points])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}