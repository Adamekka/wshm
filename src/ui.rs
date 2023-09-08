use gtk4::prelude::*;

pub trait BuildUI {
    fn build_ui(self);
}

impl BuildUI for &gtk4::Application {
    fn build_ui(self) {
        let window = gtk4::ApplicationWindow::new(self);

        gtk4_layer_shell::init_for_window(&window);
        gtk4_layer_shell::set_layer(&window, gtk4_layer_shell::Layer::Top);
        gtk4_layer_shell::auto_exclusive_zone_enable(&window);

        gtk4_layer_shell::set_margin(&window, gtk4_layer_shell::Edge::Top, 0);
        gtk4_layer_shell::set_margin(&window, gtk4_layer_shell::Edge::Bottom, 0);
        gtk4_layer_shell::set_margin(&window, gtk4_layer_shell::Edge::Left, 0);
        gtk4_layer_shell::set_margin(&window, gtk4_layer_shell::Edge::Right, 0);

        gtk4_layer_shell::set_anchor(&window, gtk4_layer_shell::Edge::Top, false);
        gtk4_layer_shell::set_anchor(&window, gtk4_layer_shell::Edge::Bottom, false);
        gtk4_layer_shell::set_anchor(&window, gtk4_layer_shell::Edge::Left, false);
        gtk4_layer_shell::set_anchor(&window, gtk4_layer_shell::Edge::Right, false);

        let label = gtk4::Label::new(Some("Shutdown Menu"));

        let lock_button = gtk4::Button::with_label("Lock");
        let sleep_button = gtk4::Button::with_label("Sleep");
        let restart_button = gtk4::Button::with_label("Restart");
        let shutdown_button = gtk4::Button::with_label("Shutdown");
        let logout_button = gtk4::Button::with_label("Logout");

        lock_button.connect_clicked(|_| {
            std::process::Command::new("loginctl")
                .arg("lock-session")
                .spawn()
                .expect("Failed to lock");
        });

        sleep_button.connect_clicked(|_| {
            std::process::Command::new("systemctl")
                .arg("suspend")
                .spawn()
                .expect("Failed to suspend");
        });

        restart_button.connect_clicked(|_| {
            std::process::Command::new("systemctl")
                .arg("reboot")
                .spawn()
                .expect("Failed to reboot");
        });

        shutdown_button.connect_clicked(|_| {
            std::process::Command::new("shutdown")
                .arg("now")
                .spawn()
                .expect("Failed to poweroff");
        });

        logout_button.connect_clicked(|_| {
            std::process::Command::new("loginctl")
                .arg("terminate-user")
                .arg("1000")
                .spawn()
                .expect("Failed to logout");
        });

        let widgets = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
        widgets.append(&label);
        widgets.append(&lock_button);
        widgets.append(&sleep_button);
        widgets.append(&restart_button);
        widgets.append(&shutdown_button);
        widgets.append(&logout_button);

        window.set_child(Some(&widgets));
        window.show();
    }
}
