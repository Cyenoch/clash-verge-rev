#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod cmds;
mod config;
mod core;
mod enhance;
mod feat;
mod utils;

use crate::utils::{init, resolve, server};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 单例检测
    if server::check_singleton().is_err() {
        println!("app exists");
        return;
    }

    crate::log_err!(init::init_config());

    #[allow(unused_mut)]
    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            resolve::resolve_setup(app);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // common
            cmds::get_sys_proxy,
            cmds::open_app_dir,
            cmds::open_logs_dir,
            cmds::open_web_url,
            cmds::open_core_dir,
            cmds::get_portable_flag,
            // cmds::kill_sidecar,
            cmds::restart_sidecar,
            cmds::grant_permission,
            // clash
            cmds::get_clash_info,
            cmds::get_clash_logs,
            cmds::patch_clash_config,
            cmds::change_clash_core,
            cmds::get_runtime_config,
            cmds::get_runtime_yaml,
            cmds::get_runtime_exists,
            cmds::get_runtime_logs,
            cmds::uwp::invoke_uwp_tool,
            // verge
            cmds::get_verge_config,
            cmds::patch_verge_config,
            cmds::test_delay,
            cmds::exit_app,
            // cmds::update_hotkeys,
            // profile
            cmds::get_profiles,
            cmds::enhance_profiles,
            cmds::patch_profiles_config,
            cmds::view_profile,
            cmds::patch_profile,
            cmds::create_profile,
            cmds::import_profile,
            cmds::reorder_profile,
            cmds::update_profile,
            cmds::delete_profile,
            cmds::read_profile_file,
            cmds::save_profile_file,
            // service mode
            cmds::service::check_service,
            cmds::service::install_service,
            cmds::service::uninstall_service,
            // clash api
            cmds::clash_api_get_proxy_delay
        ]);

    #[cfg(target_os = "macos")]
    {
        use tauri::{Menu, MenuItem, Submenu};

        builder = builder.menu(
            Menu::new().add_submenu(Submenu::new(
                "Edit",
                Menu::new()
                    .add_native_item(MenuItem::Undo)
                    .add_native_item(MenuItem::Redo)
                    .add_native_item(MenuItem::Copy)
                    .add_native_item(MenuItem::Paste)
                    .add_native_item(MenuItem::Cut)
                    .add_native_item(MenuItem::SelectAll)
                    .add_native_item(MenuItem::CloseWindow)
                    .add_native_item(MenuItem::Quit),
            )),
        );
    }

    let app = builder
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    app.run(|app_handle, e| match e {
        tauri::RunEvent::ExitRequested { code, api, .. } => {
            api.prevent_exit();
        }
        tauri::RunEvent::WindowEvent { label, event, .. } => {
            if label == "main" {
                match event {
                    tauri::WindowEvent::Destroyed => {
                        let _ = resolve::save_window_size_position(app_handle, true);
                    }
                    tauri::WindowEvent::CloseRequested { .. } => {
                        let _ = resolve::save_window_size_position(app_handle, true);
                    }
                    tauri::WindowEvent::Moved(_) | tauri::WindowEvent::Resized(_) => {
                        let _ = resolve::save_window_size_position(app_handle, false);
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    });
}