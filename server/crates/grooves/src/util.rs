use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

pub mod spotify;

const TOKEN_LENGTH: usize = 64;
pub fn generate_session_token() -> String {
    let mut rng = thread_rng();
    (&mut rng)
        .sample_iter(Alphanumeric)
        .take(TOKEN_LENGTH)
        .map(char::from)
        .collect()
}
