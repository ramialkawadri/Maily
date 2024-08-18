use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct GmailPage(ObjectSubclass<imp::GmailPage>)
        @extends adw::NavigationPage, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl GmailPage {
    pub fn new() -> Self {
        Object::builder().build()
    }
}

mod imp {
    use adw::subclass::prelude::*;
    use adw::PasswordEntryRow;
    use glib::subclass::InitializingObject;
    use glib::subclass::Signal;
    use gtk::prelude::*;
    use gtk::{glib, Button, CompositeTemplate};
    use std::sync::OnceLock;

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/org/maily/login/gmail_page.ui")]
    pub struct GmailPage {
        #[template_child]
        pub button: TemplateChild<Button>,

        #[template_child]
        pub password: TemplateChild<PasswordEntryRow>,
    }

    #[gtk::template_callbacks]
    impl GmailPage {
        #[template_callback]
        pub fn handle_password_changed(&self, entry_row: &PasswordEntryRow) {
            self.button.set_sensitive(!entry_row.text().is_empty());
        }

        #[template_callback]
        pub fn handle_login_clicked(&self, _: &Button) {
            let obj = self.obj();
            obj.emit_by_name::<()>("login-clicked", &[&self.password.text()]);
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GmailPage {
        const NAME: &'static str = "GmailPage";
        type Type = super::GmailPage;
        type ParentType = adw::NavigationPage;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for GmailPage {
        fn signals() -> &'static [Signal] {
            static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
            SIGNALS.get_or_init(|| {
                vec![Signal::builder("login-clicked")
                    // Parameters: Name, Email
                    .param_types([str::static_type()])
                    .build()]
            })
        }
    }

    impl WidgetImpl for GmailPage {}

    impl NavigationPageImpl for GmailPage {}
}
