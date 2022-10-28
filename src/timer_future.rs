use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll},
    thread,
    time::{Duration, SystemTime},
};

use crate::timer_state;

pub struct TimerFuture {
    shared_state: Arc<Mutex<timer_state::TimerState>>,
}

impl Future for TimerFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.completed {
            Poll::Ready(())
        } else {
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl TimerFuture {
    pub fn new(time: u64, start_time: SystemTime) -> TimerFuture {
        let shared_state = Arc::new(Mutex::new(timer_state::TimerState::new(
            Duration::from_secs(time),
            start_time,
        )));

        let thread_shared_state = shared_state.clone();
        thread::spawn(move || {
            for _ in 0..time {
                println!(
                    "{}",
                    timer_state::get_current_time(Duration::from_secs(time), start_time)
                        .as_secs_f64()
                        .round()
                );
                thread::sleep(Duration::from_secs(1));
            }
            let mut shared_state = thread_shared_state.lock().unwrap();
            shared_state.completed = true;
            if let Some(waker) = shared_state.waker.take() {
                waker.wake()
            }
        });

        TimerFuture { shared_state }
    }

    fn pause() {}
}
