extern crate google_gmail1 as gmail1;

use gmail1::{hyper, hyper_rustls, oauth2, Gmail};
use std::future::Future;
use std::pin::Pin;

pub struct GmailEmailClient {
    hub: Gmail<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
}

impl GmailEmailClient {
    pub async fn new() -> Result<Self, String> {
        let secret = match oauth2::read_application_secret("clientsecret.json").await {
            Ok(val) => val,
            Err(_) => return Err(String::from("Cannot find clientsecret.json")),
        };

        let auth = oauth2::InstalledFlowAuthenticator::builder(
            secret,
            oauth2::InstalledFlowReturnMethod::HTTPRedirect,
        )
        .persist_tokens_to_disk("tokencache.json")
        .flow_delegate(Box::new(InstalledFlowBrowserDelegate))
        .build()
        .await
        .unwrap();

        let hub = Gmail::new(
            hyper::Client::builder().build(
                hyper_rustls::HttpsConnectorBuilder::new()
                    .with_native_roots()
                    .unwrap()
                    .https_or_http()
                    .enable_http1()
                    .build(),
            ),
            auth,
        );

        Ok(GmailEmailClient { hub })
    }
}

impl super::EmailClient for GmailEmailClient {}

#[derive(Copy, Clone)]
struct InstalledFlowBrowserDelegate;

impl oauth2::authenticator_delegate::InstalledFlowDelegate for InstalledFlowBrowserDelegate {
    fn present_user_url<'a>(
        &'a self,
        url: &'a str,
        need_code: bool,
    ) -> Pin<Box<dyn Future<Output = Result<String, String>> + Send + 'a>> {
        let _ = open::that(url);
        oauth2::authenticator_delegate::DefaultInstalledFlowDelegate
            .present_user_url(url, need_code)
    }
}
