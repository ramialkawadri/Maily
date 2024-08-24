use glib::Object;
use gtk::{glib, gio};

glib::wrapper! {
    pub struct MainWindow(ObjectSubclass<imp::MainWindow>)
        @extends adw::ApplicationWindow, gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl MainWindow {
    pub fn new(app: &adw::Application) -> Self {
        Object::builder().property("application", app).build()
    }
}

mod imp {
    use adw::subclass::prelude::*;
    use glib::subclass::InitializingObject;
    use gtk::{glib, CompositeTemplate};
    use std::default::Default;

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/org/maily/main_window/window.ui")]
    pub struct MainWindow {}

    #[glib::object_subclass]
    impl ObjectSubclass for MainWindow {
        const NAME: &'static str = "MainWindow";
        type Type = super::MainWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for MainWindow {}

    impl WidgetImpl for MainWindow {}

    impl WindowImpl for MainWindow {}

    impl AdwWindowImpl for MainWindow {}

    impl ApplicationWindowImpl for MainWindow {}

    impl AdwApplicationWindowImpl for MainWindow {}
}
