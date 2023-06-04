#![cfg_attr(
  all(windows, not(debug_assertions)), 
  windows_subsystem = "windows"
)]

fn main() {
  kubi::kubi_main();
}
