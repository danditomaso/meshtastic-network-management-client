use crate::state;
use log::{error, info};
use tauri::Manager;

#[derive(Debug, serde::Deserialize)]
pub struct CliConfig {
    port: Option<String>,
}

pub fn handle_cli_args(
    app: &tauri::App,
    initial_autoconnect_state: &mut state::autoconnect::AutoConnectState,
) -> Result<(), String> {
    // Get CLI args from app configuration context
    let matches = app.env().args();

    // Look for --port or -p argument
    for (i, arg) in matches.iter().enumerate() {
        if arg == "--port" || arg == "-p" {
            // Try to get the next argument as the port value
            if let Some(port_name) = matches.get(i + 1) {
                info!("Found port CLI argument: {}", port_name);
                *initial_autoconnect_state =
                    state::autoconnect::AutoConnectState::init(port_name.to_string());
                return Ok(());
            }
        }
    }

    info!("No port CLI argument specified, skipping...");
    Ok(())
}
