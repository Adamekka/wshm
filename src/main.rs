mod ui;

use crate::ui::BuildUI;
use gtk4::prelude::*;

fn main() {
    let app = gtk4::Application::new(Some("com.adamekka.wshm"), Default::default());

    app.connect_activate(|app| {
        app.build_ui();
    });

    app.run();
}
