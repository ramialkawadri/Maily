mod name_email_page;
mod gmail_page;

use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct LoginDialog(ObjectSubclass<imp::LoginDialog>)
        @extends adw::Dialog, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl LoginDialog {
    pub fn new() -> Self {
        Object::builder().build()
    }
}

mod imp {
    use adw::subclass::prelude::*;
    use glib::subclass::InitializingObject;
    use gtk::{glib, CompositeTemplate};
    use adw::NavigationView;
    use std::cell::RefCell;

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/org/maily/login/dialog.ui")]
    pub struct LoginDialog {
        #[template_child]
        pub name_email_page: TemplateChild<super::name_email_page::NameEmailPage>,

        #[template_child]
        pub gmail_page: TemplateChild<super::gmail_page::GmailPage>,

        #[template_child]
        pub navigation_view: TemplateChild<NavigationView>,

        name: RefCell<String>,
        email: RefCell<String>,
    }

    #[gtk::template_callbacks]
    impl LoginDialog {
        #[template_callback]
        fn handle_next_clicked(&self, name: &str, email: &str) {
            self.navigation_view.push(&self.gmail_page.get());
            *self.name.borrow_mut() = String::from(name);
            *self.email.borrow_mut() = String::from(email);
        }

        #[template_callback]
        fn handle_login_clicked(&self, _password: &str) {
            glib::g_message!("Auth", "Logging in {}.", self.email.borrow());
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for LoginDialog {
        const NAME: &'static str = "LoginDialog";
        type Type = super::LoginDialog;
        type ParentType = adw::Dialog;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for LoginDialog {}

    impl WidgetImpl for LoginDialog {}

    impl AdwDialogImpl for LoginDialog {}
}

