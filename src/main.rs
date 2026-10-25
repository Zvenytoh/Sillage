#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod artwork;
mod cache;
mod catalog;
mod library;
mod platform;
mod session;
mod view;

fn main() {
    if let Err(error) = app::run() {
        platform::file_dialog::show_error(&error.to_string());
        std::process::exit(1);
    }
}
