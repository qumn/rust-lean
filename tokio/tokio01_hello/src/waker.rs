use std::{sync::{Arc, Mutex}, task::Waker};

pub struct TimerFuture {
    share_state: Arc<Mutex<ShareState>>,
}

struct ShareState {
    completed: bool,
    waker: Option<Waker>,
}
