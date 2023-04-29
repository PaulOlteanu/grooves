use rspotify::ClientError;

#[derive(Debug)]
pub enum PlayerError {
    ChannelError,
    CommandError,
    NoPlayback,
    SpotifyError,
    TooManyErrors,
    OtherError(String),
}

impl From<ClientError> for PlayerError {
    fn from(_value: ClientError) -> Self {
        Self::SpotifyError
    }
}
