use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct FirstPage(ObjectSubclass<imp::FirstPage>)
        @extends adw::NavigationPage, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl FirstPage {
    pub fn new() -> Self {
        Object::builder().build()
    }
}

mod imp {
    use adw::subclass::prelude::*;
    use adw::{EntryRow, ComboRow};
    use glib::subclass::InitializingObject;
    use glib::subclass::Signal;
    use gtk::prelude::*;
    use gtk::{glib, Button, CompositeTemplate};
    use std::sync::OnceLock;
    use crate::core::email_client::ProviderType;

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/org/maily/login/first_page.ui")]
    pub struct FirstPage {
        #[template_child]
        pub button: TemplateChild<Button>,

        #[template_child]
        pub name: TemplateChild<EntryRow>,

        #[template_child]
        pub provider: TemplateChild<ComboRow>,
    }

    #[gtk::template_callbacks]
    impl FirstPage {
        #[template_callback]
        fn handle_email_name_changed(&self, _: &EntryRow) {
            self.button.set_sensitive(self.should_enable_next_button());
        }

        fn should_enable_next_button(&self) -> bool {
            !self.name.text().is_empty()
        }

        #[template_callback]
        fn handle_next_clicked(&self, _: &Button) {
            let obj = self.obj();
            obj.emit_by_name::<()>("next-clicked", &[&self.name.text(), &ProviderType::GMail]);
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for FirstPage {
        const NAME: &'static str = "FirstPage";
        type Type = super::FirstPage;
        type ParentType = adw::NavigationPage;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for FirstPage {
        fn signals() -> &'static [Signal] {
            static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
            SIGNALS.get_or_init(|| {
                vec![Signal::builder("next-clicked")
                    // Parameters: Name, Provider
                    .param_types([str::static_type(), ProviderType::static_type()])
                    .build()]
            })
        }
    }

    impl WidgetImpl for FirstPage {}

    impl NavigationPageImpl for FirstPage {}
}
