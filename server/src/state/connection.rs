use std::sync::{Arc, Mutex};
use std::task::{Poll, Waker};

use futures::Future;

use super::State;
use crate::player_connection::PlayerConnection;

pub struct SharedState {
    pub conn: Option<PlayerConnection>,
    pub waker: Option<Waker>,
}

pub struct FutureConnection {
    shared_state: Arc<Mutex<SharedState>>,
}

impl FutureConnection {
    pub async fn new(state: &State, user_id: i32) -> Self {
        if let Some(player) = state.get_player(&user_id).await {
            Self {
                shared_state: Arc::new(Mutex::new(SharedState {
                    conn: Some(player),
                    waker: None,
                })),
            }
        } else {
            let shared_state = Self::register(state, user_id);
            Self { shared_state }
        }
    }

    fn register(state: &State, user_id: i32) -> Arc<Mutex<SharedState>> {
        let mut awaiting = state.awaiting_player.lock().unwrap();
        let values = awaiting.entry(user_id).or_insert_with(Vec::new);
        let shared_state = Arc::new(Mutex::new(SharedState {
            conn: None,
            waker: None,
        }));

        values.push(shared_state.clone());

        shared_state
    }
}

impl Future for FutureConnection {
    type Output = PlayerConnection;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock().unwrap();
        if let Some(conn) = shared_state.conn.clone() {
            Poll::Ready(conn)
        } else {
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}
