use std::sync::Arc;

use rspotify::sync::Mutex;
use rspotify::{scopes, AuthCodeSpotify, Config, Credentials, OAuth, Token};

use crate::models::spotify::SpotifyCreds;

pub mod fetcher;

/// This will return a spotify client without any token
pub fn init_client() -> AuthCodeSpotify {
    let creds = Credentials::from_env().unwrap();

    let scopes = scopes!(
        "user-read-currently-playing",
        "user-modify-playback-state",
        "user-read-playback-state",
        "playlist-read-private",
        "user-read-private"
    );

    let oauth = OAuth::from_env(scopes).unwrap();

    AuthCodeSpotify::new(creds, oauth)
}

pub fn client_with_token(mut token: Token) -> AuthCodeSpotify {
    let creds = Credentials::from_env().unwrap();

    let scopes = scopes!(
        "user-read-currently-playing",
        "user-modify-playback-state",
        "user-read-playback-state",
        "playlist-read-private",
        "user-read-private"
    );

    let oauth = OAuth::from_env(scopes.clone()).unwrap();

    let config: Config = Config {
        token_refreshing: true,
        ..Default::default()
    };

    token.scopes = scopes;

    let mut client = AuthCodeSpotify::with_config(creds, oauth, config);
    client.token = Arc::new(Mutex::new(Some(token)));
    client
}

pub fn init_client_with_creds(token: SpotifyCreds) -> AuthCodeSpotify {
    let creds = Credentials::from_env().unwrap();

    let scopes = scopes!(
        "user-read-currently-playing",
        "user-modify-playback-state",
        "user-read-playback-state",
        "playlist-read-private",
        "user-read-private"
    );

    let oauth = OAuth::from_env(scopes.clone()).unwrap();

    let mut client = AuthCodeSpotify::new(creds, oauth);
    let token = Token {
        access_token: token.access_token,
        refresh_token: token.refresh_token,
        scopes,
        ..Default::default()
    };

    client.token = Arc::new(Mutex::new(Some(token)));

    client
}
