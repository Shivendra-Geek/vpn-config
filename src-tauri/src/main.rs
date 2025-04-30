use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use std::thread;
use tauri::{Manager};
use tauri::path::BaseDirectory;
use std::path::PathBuf;
use std::time::{Duration, Instant};

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      let app_handle = app.handle();
      
      // Get the resource directory where DLLs and exe are stored
      let resource_dir = app_handle
        .path()
        .resolve("vpn", BaseDirectory::Resource)
        .expect("❌ Failed to find resource directory");
      
      // Resolve paths
      let openvpn_path = resource_dir.join("openvpn.exe");
      let config_path = resource_dir.join("openvpn.ovpn");
      
      // Debug paths
      println!("▶ Resource dir: {}", resource_dir.display());
      println!("▶ OpenVPN path: {}", openvpn_path.display());
      println!("▶ Config path: {}", config_path.display());
      
      // Check if files exist
      if !openvpn_path.exists() {
        println!("❌ OpenVPN binary not found at: {}", openvpn_path.display());
        return Ok(());
      }
      if !config_path.exists() {
        println!("❌ Config file not found at: {}", config_path.display());
        return Ok(());
      }
      
      // Check IP address before VPN connection
      println!("🔍 Checking IP address before VPN connection...");
      if let Ok(output) = Command::new("cmd")
        .args(&["/c", "curl", "-s", "https://api.ipify.org"])
        .output() {
        if output.status.success() {
          let ip = String::from_utf8_lossy(&output.stdout);
          println!("📍 Current IP (before VPN): {}", ip);
        } else {
          println!("❌ Failed to get IP address before VPN. Error: {:?}", output.stderr);
        }
      } else {
        println!("❌ Failed to execute curl command to get IP address before VPN.");
      }
      
      // IMPORTANT: Set working directory to where the DLLs are
      // Spawn OpenVPN with stdout capture for monitoring
      println!("🔄 Starting OpenVPN...");
      let child = Command::new(&openvpn_path)
        .current_dir(&resource_dir) // This is critical - sets working dir to where DLLs are
        .arg("--config")
        .arg(&config_path)
        .stdout(Stdio::piped())
        .spawn();

      match child {
        Ok(mut child) => {
          println!("🚀 OpenVPN started with PID: {}", child.id());
          
          // Capture stdout for monitoring
          let stdout = child.stdout.take().expect("Failed to capture stdout");
          let reader = BufReader::new(stdout);
          
          // Start a thread to monitor OpenVPN output and VPN status
          thread::spawn(move || {
            let mut vpn_connected = false;
            
            println!("📊 VPN Connection Monitor Started");
            println!("----------------------------------");
            
            for line in reader.lines() {
              if let Ok(line) = line {
                // Print the OpenVPN output
                println!("🔶 OpenVPN: {}", line);
                
                // Look for connection success message
                if line.contains("Initialization Sequence Completed") {
                  vpn_connected = true;
                  println!("\n✅ VPN CONNECTED SUCCESSFULLY!");
                  println!("----------------------------------");
                  
                  // Check IP after VPN connection (wait a moment for connection to stabilize)
                  thread::sleep(Duration::from_secs(2));
                  
                  if let Ok(output) = Command::new("cmd")
                    .args(&["/c", "curl", "-s", "https://api.ipify.org"])
                    .output() {
                    if output.status.success() {
                      let ip = String::from_utf8_lossy(&output.stdout);
                      println!("📍 Current IP (through VPN): {}", ip);
                      println!("----------------------------------");
                    } else {
                      println!("❌ Failed to get IP address after VPN connection. Error: {:?}", output.stderr);
                    }
                  } else {
                    println!("❌ Failed to execute curl command to get IP address after VPN.");
                  }
                  
                  // Check route table to verify VPN routes
                  println!("\n🔍 Checking network routes...");
                  if let Ok(output) = Command::new("cmd")
                    .args(&["/c", "route", "print"])
                    .output() {
                    if output.status.success() {
                      let routes = String::from_utf8_lossy(&output.stdout);
                      println!("🔀 Network Routes (showing first 20 lines):");
                      for (i, line) in routes.lines().enumerate() {
                        if i < 20 {
                          println!("  {}", line);
                        } else if i == 20 {
                          println!("  ... (more routes not shown)");
                          break;
                        }
                      }
                    } else {
                      println!("❌ Failed to get network routes. Error: {:?}", output.stderr);
                    }
                  } else {
                    println!("❌ Failed to execute route command to get network routes.");
                  }
                }
                
                // Look for disconnection or errors
                if line.contains("Connection reset") || line.contains("AUTH_FAILED") {
                  vpn_connected = false;
                  println!("\n❌ VPN DISCONNECTED!");
                  println!("----------------------------------");
                }
              }
            }
            
            println!("📊 OpenVPN process ended");
          });
        }
        Err(e) => {
          println!("❌ Failed to start OpenVPN: {}", e);
        }
      }
      
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("Tauri error");
}