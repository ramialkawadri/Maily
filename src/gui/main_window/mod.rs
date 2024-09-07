use glib::Object;
use gtk::{gio, glib};

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
    use gtk::prelude::*;
    use gtk::{glib, Box, CompositeTemplate, Image, Inscription, Label, ListBox};
    use std::default::Default;

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/org/maily/main_window/window.ui")]
    pub struct MainWindow {
        #[template_child]
        pub folders_list: TemplateChild<ListBox>,
    }

    impl MainWindow {
        fn add_folder(&self, icon_name: String, label: String, unread: i32, attention: bool) {
            let gtk_box = Box::builder()
                .orientation(gtk::Orientation::Horizontal)
                .css_classes(vec!["folder-row"])
                .build();

            let image = Image::from_icon_name(icon_name.as_str());
            gtk_box.append(&image);

            let label_widget = Inscription::builder()
                .text(label)
                .hexpand(true)
                .xalign(0f32)
                .text_overflow(gtk::InscriptionOverflow::EllipsizeEnd)
                .build();
            gtk_box.append(&label_widget);

            if unread > 0 {
                let mut css_classes = vec!["counter", "numeric"];

                if attention {
                    css_classes.push("needs-attention");
                }

                if unread >= 10 {
                    css_classes.push("long");
                }

                let counter = Label::builder()
                    .label(unread.to_string())
                    .valign(gtk::Align::Center)
                    .css_classes(css_classes)
                    .build();
                gtk_box.append(&counter);
            }

            self.folders_list.append(&gtk_box);
        }
    }

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

    impl ObjectImpl for MainWindow {
        fn constructed(&self) {
            self.parent_constructed();
        }
    }

    impl WidgetImpl for MainWindow {}

    impl WindowImpl for MainWindow {}

    impl AdwWindowImpl for MainWindow {}

    impl ApplicationWindowImpl for MainWindow {}

    impl AdwApplicationWindowImpl for MainWindow {}
}
