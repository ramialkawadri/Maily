/*
//!
//! This example showcases the Google OAuth2 process for requesting access to the Google Calendar features
//! and the user's profile.
//!
//! Before running it, you'll need to generate your own Google OAuth2 credentials.
//!
//! In order to run the example call:
//!
//! ```sh
//! GOOGLE_CLIENT_ID=xxx GOOGLE_CLIENT_SECRET=yyy cargo run --example google
//! ```
//!
//! ...and follow the instructions.
//!
*/
use oauth2::reqwest;
use oauth2::{basic::BasicClient, StandardRevocableToken, TokenResponse};
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl,
    RevocationUrl, Scope, TokenUrl,
};
use url::Url;

use std::env;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;

const GOOGLE_AUTH_URL: &str = "https://accounts.google.com/o/oauth2/v2/auth";
const GOOGLE_TOKEN_URL: &str = "https://www.googleapis.com/oauth2/v3/token";
const GOOGLE_REVOKE_URL: &str = "https://oauth2.googleapis.com/revoke";

pub struct GmailEmailClient {}

impl super::EmailClient for GmailEmailClient {
    fn authenticate(&self) -> Result<(), String> {
        let google_client_id = match env::var("GOOGLE_CLIENT_ID") {
            Ok(client_id) => ClientId::new(client_id),
            Err(_) => {
                return Err(String::from(
                    "Missing the GOOGLE_CLIENT_ID environment variable.",
                ))
            }
        };
        let google_client_secret = match env::var("GOOGLE_CLIENT_SECRET") {
            Ok(client_secret) => ClientSecret::new(client_secret),
            Err(_) => {
                return Err(String::from(
                    "Missing the GOOGLE_CLIENT_SECRET environment variable.",
                ))
            }
        };
        let auth_url = AuthUrl::new(GOOGLE_AUTH_URL.to_string()).unwrap();
        let token_url = TokenUrl::new(GOOGLE_TOKEN_URL.to_string()).unwrap();

        let client = BasicClient::new(
            google_client_id,
            Some(google_client_secret),
            auth_url,
            Some(token_url),
        )
        .set_redirect_uri(RedirectUrl::new("http://localhost:8080".to_string()).unwrap())
        .set_revocation_uri(RevocationUrl::new(GOOGLE_REVOKE_URL.to_string()).unwrap());

        let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();
        let (authorize_url, _) = client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new(
                "https://www.googleapis.com/auth/gmail.readonly".to_string(),
            ))
            .set_pkce_challenge(pkce_code_challenge)
            .url();

        let _ = open::that(authorize_url.as_str());
        println!("Open this URL in your browser:\n{authorize_url}\n");

        let code = {
            let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
            let Some(mut stream) = listener.incoming().flatten().next() else {
                return Err(String::from(
                    "Listener terminated without accepting a connection",
                ));
            };

            let mut reader = BufReader::new(&stream);

            let mut request_line = String::new();
            reader.read_line(&mut request_line).unwrap();

            let redirect_url = request_line.split_whitespace().nth(1).unwrap();
            let url = Url::parse(&("http://localhost".to_string() + redirect_url)).unwrap();

            let error_param = url
                .query_pairs()
                .find(|(key, _)| key == "error")
                .map(|(_, error)| error.into_owned());

            if let Some(error) = error_param {
                return Err(format!("An error has happened: '{}'", error));
            }

            let code = url
                .query_pairs()
                .find(|(key, _)| key == "code")
                .map(|(_, code)| AuthorizationCode::new(code.into_owned()))
                .unwrap();

            let message = "Login succeeded";
            let response = format!(
                "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
                message.len(),
                message
            );
            stream.write_all(response.as_bytes()).unwrap();

            code
        };

        // Exchange the code with a token.
        let token_response = client
            .exchange_code(code)
            .set_pkce_verifier(pkce_code_verifier)
            .request(reqwest::http_client);

        // Revoke the obtained token.
        let token_response = token_response.unwrap();
        let token_to_revoke: StandardRevocableToken = match token_response.refresh_token() {
            Some(token) => token.into(),
            None => token_response.access_token().into(),
        };

        client
            .revoke_token(token_to_revoke)
            .unwrap()
            .request(reqwest::http_client)
            .expect("Failed to revoke token");

        Ok(())
    }
}
