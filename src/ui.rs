use crate::config::Config;
use gtk4::prelude::*;

pub trait BuildUI {
    fn build_ui(self, config: &Config);
}

impl BuildUI for &gtk4::Application {
    fn build_ui(self, config: &Config) {
        let window = gtk4::ApplicationWindow::new(self);

        gtk4_layer_shell::init_for_window(&window);
        gtk4_layer_shell::set_layer(&window, gtk4_layer_shell::Layer::Top);
        gtk4_layer_shell::auto_exclusive_zone_enable(&window);

        gtk4_layer_shell::set_margin(&window, gtk4_layer_shell::Edge::Top, config.margins.top);
        gtk4_layer_shell::set_margin(
            &window,
            gtk4_layer_shell::Edge::Bottom,
            config.margins.bottom,
        );
        gtk4_layer_shell::set_margin(&window, gtk4_layer_shell::Edge::Left, config.margins.left);
        gtk4_layer_shell::set_margin(&window, gtk4_layer_shell::Edge::Right, config.margins.right);

        gtk4_layer_shell::set_anchor(&window, gtk4_layer_shell::Edge::Top, false);
        gtk4_layer_shell::set_anchor(&window, gtk4_layer_shell::Edge::Bottom, false);
        gtk4_layer_shell::set_anchor(&window, gtk4_layer_shell::Edge::Left, false);
        gtk4_layer_shell::set_anchor(&window, gtk4_layer_shell::Edge::Right, false);

        let label = gtk4::Label::new(Some(config.title.as_str()));

        let widgets = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
        widgets.append(&label);

        for button in config.buttons.clone() {
            let gtk_button: gtk4::Button = gtk4::Button::with_label(button.label.as_str());

            gtk_button.connect_clicked(move |_| {
                let mut command = std::process::Command::new(&button.actions[0]);
                for action in &button.actions[1..] {
                    command.arg(action);
                }
                command.spawn().expect("Failed to run command");
            });

            widgets.append(&gtk_button);
        }

        window.set_child(Some(&widgets));
        window.show();
    }
}
