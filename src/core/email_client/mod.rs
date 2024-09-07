mod gmail;

use gtk::glib;
use std::pin::Pin;
use std::future::Future;
use std::marker::Send;

pub trait EmailClient {
    fn get_email(&self) -> Pin<Box<dyn Future<Output = String> + Send + '_>>;
}

pub async fn get_email_client(provider_type: ProviderType) -> Result<Box<dyn EmailClient>, String> {
    match provider_type {
        ProviderType::GMail => {
            let result = gmail::GmailEmailClient::new().await?;
            Ok(Box::new(result))
        },
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, glib::Enum)]
#[enum_type(name = "ProviderType")]
pub enum ProviderType {
    #[enum_value(name = "GMail")]
    GMail,
}
