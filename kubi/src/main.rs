#![cfg_attr(
  all(windows, not(debug_assertions)),
  windows_subsystem = "windows"
)]

fn main() {
  kubimain::kubi_main()
}
