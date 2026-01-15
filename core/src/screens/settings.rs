pub mod about_panel;
pub mod add_provider_modal;
pub mod audio_panel;
pub mod general_panel;
pub mod provider_view;
pub mod providers_panel;
pub mod settings_screen;

pub use audio_panel::AudioDevices;
use makepad_widgets::Cx;
pub use settings_screen::{SettingsScreen, SettingsScreenAction, SettingsScreenWidgetRefExt, ThemeMode};

pub(super) fn live_design(cx: &mut Cx) {
    about_panel::live_design(cx);
    add_provider_modal::live_design(cx);
    audio_panel::live_design(cx);
    general_panel::live_design(cx);
    provider_view::live_design(cx);
    providers_panel::live_design(cx);
    settings_screen::live_design(cx);
}

/// Get the default data location path
pub fn get_default_data_location() -> String {
    dirs::document_dir()
        .map(|p: std::path::PathBuf| p.join("colang").to_string_lossy().to_string())
        .unwrap_or_else(|| "~/Documents/colang".to_string())
}

/// Open the data location in the system file explorer
pub fn open_data_location(data_location: &str) {
    use std::process::Command;

    let path = if data_location.is_empty() {
        get_default_data_location()
    } else {
        data_location.to_string()
    };

    // Create directory if it doesn't exist
    let _ = std::fs::create_dir_all(&path);

    // Open file explorer based on OS
    #[cfg(target_os = "windows")]
    {
        let _ = Command::new("explorer").arg(&path).spawn();
    }

    #[cfg(target_os = "macos")]
    {
        let _ = Command::new("open").arg(&path).spawn();
    }

    #[cfg(target_os = "linux")]
    {
        let _ = Command::new("xdg-open").arg(&path).spawn();
    }
}

/// Clear the application cache directory
pub fn clear_cache() -> Result<(), std::io::Error> {
    if let Some(cache_dir) = dirs::cache_dir() {
        let app_cache = cache_dir.join("colang");
        if app_cache.exists() {
            std::fs::remove_dir_all(&app_cache)?;
        }
    }
    Ok(())
}

/// Open a folder picker dialog and return the selected path
pub fn browse_data_location(current_location: &str) -> Option<String> {
    use std::path::PathBuf;

    // Get the current data location as starting point
    let start_dir = if current_location.is_empty() {
        dirs::document_dir().unwrap_or_else(|| PathBuf::from("."))
    } else {
        PathBuf::from(current_location)
    };

    // Spawn the dialog in a blocking context since rfd dialogs are blocking on desktop
    let dialog = rfd::FileDialog::new()
        .set_title("Select Data Location")
        .set_directory(&start_dir);

    dialog
        .pick_folder()
        .map(|folder: std::path::PathBuf| folder.to_string_lossy().to_string())
}

/// Initialize and enumerate audio devices using cpal
pub fn init_audio_devices() -> AudioDevices {
    use cpal::traits::{DeviceTrait, HostTrait};

    let host = cpal::default_host();

    // Get input devices
    let default_input_name = host.default_input_device().and_then(|d| d.name().ok());
    let mut input_labels = Vec::new();
    let mut input_devices = Vec::new();

    if let Ok(inputs) = host.input_devices() {
        for device in inputs {
            if let Ok(name) = device.name() {
                let is_default = default_input_name.as_ref().map_or(false, |d| d == &name);
                let label = if is_default {
                    format!("Default ({})", name)
                } else {
                    name.clone()
                };
                input_labels.push(label);
                input_devices.push(name);
            }
        }
    }

    // Sort with default first
    if !input_labels.is_empty() {
        let default_idx = input_labels.iter().position(|l| l.starts_with("Default ("));
        if let Some(idx) = default_idx {
            if idx != 0 {
                input_labels.swap(0, idx);
                input_devices.swap(0, idx);
            }
        }
    }

    // Get output devices
    let default_output_name = host.default_output_device().and_then(|d| d.name().ok());
    let mut output_labels = Vec::new();
    let mut output_devices = Vec::new();

    if let Ok(outputs) = host.output_devices() {
        for device in outputs {
            if let Ok(name) = device.name() {
                let is_default = default_output_name.as_ref().map_or(false, |d| d == &name);
                let label = if is_default {
                    format!("Default ({})", name)
                } else {
                    name.clone()
                };
                output_labels.push(label);
                output_devices.push(name);
            }
        }
    }

    // Sort with default first
    if !output_labels.is_empty() {
        let default_idx = output_labels
            .iter()
            .position(|l| l.starts_with("Default ("));
        if let Some(idx) = default_idx {
            if idx != 0 {
                output_labels.swap(0, idx);
                output_devices.swap(0, idx);
            }
        }
    }

    AudioDevices {
        input_devices,
        output_devices,
        input_labels,
        output_labels,
    }
}
