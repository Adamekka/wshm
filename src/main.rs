mod config;
mod ui;

use crate::ui::BuildUI;
use config::Config;
use gtk4::prelude::*;
use std::path::Path;

fn main() {
    let home_var = std::env::var("HOME").expect("Failed to get $HOME");
    let config_path = Path::new(&home_var)
        .join(".config")
        .join("wshm")
        .join("config.jsonc");

    println!("Trying to use: {:?}", config_path);

    let config = if config_path.exists() {
        Config::load(&config_path).unwrap_or_default()
    } else {
        eprintln!("Config file not found, using default config");
        Config::default()
    };

    let app = gtk4::Application::new(Some("com.adamekka.wshm"), Default::default());

    app.connect_activate(move |app| {
        app.build_ui(&config);
    });

    app.run();
}
