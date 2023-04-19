use std::sync::Arc;

use rspotify::sync::Mutex;
use rspotify::{scopes, AuthCodeSpotify, Config, Credentials, OAuth, Token};

pub fn client_with_token(mut token: Token) -> AuthCodeSpotify {
    let creds = Credentials::from_env().unwrap();

    let scopes = scopes!(
        "user-read-currently-playing",
        "user-modify-playback-state",
        "user-read-playback-state",
        "playlist-read-private",
        "user-read-private"
    );

    let mut oauth = OAuth::default();
    oauth.scopes = scopes.clone();

    let config: Config = Config {
        token_refreshing: true,
        ..Default::default()
    };

    token.scopes = scopes;

    let mut client = AuthCodeSpotify::with_config(creds, oauth, config);
    client.token = Arc::new(Mutex::new(Some(token)));
    client
}
