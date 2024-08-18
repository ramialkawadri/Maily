mod gui;

use adw::prelude::*;
use gtk::{gio, glib, CssProvider, gdk};
use adw::{HeaderBar, Window};
use gui::*;

const APP_ID: &str = "org.maily";

fn main() -> glib::ExitCode {
    gio::resources_register_include!("resources.gresource")
        .expect("Failed to register resources.");

    let app = adw::Application::builder().application_id(APP_ID).build();
    app.connect_startup(|_| {
        load_css()
    });
    app.connect_activate(build_ui);
    app.run()
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_resource("/org/maily/style.css");

    gtk::style_context_add_provider_for_display(
        &gdk::Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_ui(app: &adw::Application) {
    // TODO: fix
    let header = HeaderBar::builder().build();
    let window = Window::builder()
        .application(app)
        .width_request(800)
        .height_request(800)
        .content(&header)
        .build();

    let dialog = login::LoginDialog::new();
    dialog.present(Some(&window));

    window.present();
}
