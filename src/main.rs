#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod platform;
mod search;

fn main() {
    if let Err(error) = app::run() {
        platform::file_dialog::show_error(&error.to_string());
        std::process::exit(1);
    }
}
