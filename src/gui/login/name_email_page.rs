use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct NameEmailPage(ObjectSubclass<imp::NameEmailPage>)
        @extends adw::NavigationPage, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl NameEmailPage {
    pub fn new() -> Self {
        Object::builder().build()
    }
}

mod imp {
    use adw::subclass::prelude::*;
    use adw::EntryRow;
    use glib::subclass::InitializingObject;
    use glib::subclass::Signal;
    use gtk::prelude::*;
    use gtk::{glib, Button, CompositeTemplate};
    use std::sync::OnceLock;

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/org/maily/login/name_email_page.ui")]
    pub struct NameEmailPage {
        #[template_child]
        pub button: TemplateChild<Button>,

        #[template_child]
        pub name: TemplateChild<EntryRow>,

        #[template_child]
        pub email: TemplateChild<EntryRow>,
    }

    #[gtk::template_callbacks]
    impl NameEmailPage {
        #[template_callback]
        fn handle_email_name_changed(&self, _: &EntryRow) {
            self.button.set_sensitive(self.should_enable_next_button());
        }

        fn should_enable_next_button(&self) -> bool {
            self.email.text().contains('@')
                && self.email.text().contains('.')
                && !self.name.text().is_empty()
        }

        #[template_callback]
        fn handle_next_clicked(&self, _: &Button) {
            let obj = self.obj();
            obj.emit_by_name::<()>("next-clicked", &[&self.name.text(), &self.email.text()]);
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for NameEmailPage {
        const NAME: &'static str = "NameEmailPage";
        type Type = super::NameEmailPage;
        type ParentType = adw::NavigationPage;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for NameEmailPage {
        fn signals() -> &'static [Signal] {
            static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
            SIGNALS.get_or_init(|| {
                vec![Signal::builder("next-clicked")
                    // Parameters: Name, Email
                    .param_types([str::static_type(), str::static_type()])
                    .build()]
            })
        }
    }

    impl WidgetImpl for NameEmailPage {}

    impl NavigationPageImpl for NameEmailPage {}
}
