mod first_page;

use adw::prelude::*;
use adw::Toast;
use glib::Object;
use gtk::glib;
use gtk::subclass::prelude::*;

use crate::core::email_client;
use crate::core::email_client::ProviderType;

glib::wrapper! {
    pub struct LoginDialog(ObjectSubclass<imp::LoginDialog>)
        @extends adw::Dialog, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

#[gtk::template_callbacks]
impl LoginDialog {
    pub fn new() -> Self {
        Object::builder().build()
    }

    #[template_callback]
    async fn handle_next_clicked(&self, _name: &str, provider: ProviderType) {
        let client = email_client::get_email_client(provider);
        if let Err(error) = client.await {
            let toast = Toast::builder().title(error).build();
            self.imp().toast_overlay.add_toast(toast);
        }
        self.close();
    }
}

mod imp {
    use adw::subclass::prelude::*;
    use adw::{NavigationView, ToastOverlay};
    use glib::subclass::InitializingObject;
    use gtk::{glib, CompositeTemplate};
    use std::cell::RefCell;
    use std::default::Default;

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/org/maily/login/dialog.ui")]
    pub struct LoginDialog {
        #[template_child]
        pub first_page: TemplateChild<super::first_page::FirstPage>,

        #[template_child]
        pub navigation_view: TemplateChild<NavigationView>,

        #[template_child]
        pub toast_overlay: TemplateChild<ToastOverlay>,

        pub name: RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for LoginDialog {
        const NAME: &'static str = "LoginDialog";
        type Type = super::LoginDialog;
        type ParentType = adw::Dialog;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_instance_callbacks();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for LoginDialog {}

    impl WidgetImpl for LoginDialog {}

    impl AdwDialogImpl for LoginDialog {}
}
