use std::time::SystemTime;
use wasm_bindgen::prelude::wasm_bindgen;

mod executor;
mod timer_future;
mod timer_state;

#[wasm_bindgen]
pub fn start_timer(time: usize) {
    let (executor, spawner) = executor::new_executor_and_spawner();
    {
        spawner.spawn(async move {
            let start_time = SystemTime::now();
            timer_future::TimerFuture::new(time as u64, start_time).await;
        })
    };
    drop(spawner);
    executor.run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        start_timer(60);
    }
}
