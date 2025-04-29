use std::process::Command;
use tauri::{Manager};
use tauri::path::BaseDirectory;

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      let app_handle = app.handle();

      let openvpn_path = app_handle
        .path()
        .resolve("vpn/openvpn.exe", BaseDirectory::Resource)
        .expect("Failed to find OpenVPN binary");

      let config_path = app_handle
        .path()
        .resolve("vpn/vpngate_public-vpn-185.opengw.net_tcp_443.ovpn", BaseDirectory::Resource)
        .expect("Failed to find OpenVPN config");

      Command::new(openvpn_path)
        .arg("--config")
        .arg(config_path)
        .spawn()
        .expect("Failed to start OpenVPN");

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("Tauri error");
}
