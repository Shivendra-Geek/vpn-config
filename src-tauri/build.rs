fn main() {
  // Add manifest settings as environment variables that Tauri can use
  println!("cargo:rustc-env=TAURI_WINDOWS_MANIFEST_PATH=app.manifest");
  
  // Call the tauri build process
  tauri_build::build();
}