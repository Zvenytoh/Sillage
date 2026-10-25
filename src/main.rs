#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod artwork;
mod library;
mod platform;
mod search;
mod session;

fn main() {
    if let Err(error) = app::run() {
        platform::file_dialog::show_error(&error.to_string());
        std::process::exit(1);
    }
}
