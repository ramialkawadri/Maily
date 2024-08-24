mod gmail;

use gtk::glib;

pub trait EmailClient {
}

pub async fn get_email_client(provider_type: ProviderType) -> Result<impl EmailClient, String> {
    match provider_type {
        ProviderType::GMail => gmail::GmailEmailClient::new().await,
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, glib::Enum)]
#[enum_type(name = "ProviderType")]
pub enum ProviderType {
    #[enum_value(name = "GMail")]
    GMail,
}
