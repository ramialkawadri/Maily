mod gmail;

use gtk::glib;

pub trait EmailClient {
    fn authenticate(&self) -> Result<(), String>;
}

pub fn get_email_client(provider_type: ProviderType) -> impl EmailClient {
    match provider_type {
        ProviderType::GMail => gmail::GmailEmailClient {},
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, glib::Enum)]
#[enum_type(name = "ProviderType")]
pub enum ProviderType {
    #[enum_value(name = "GMail")]
    GMail,
}
