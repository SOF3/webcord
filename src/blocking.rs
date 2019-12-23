use actix_threadpool::{self as atp, BlockingError};

pub async fn block<T, F>(f: F) -> T
where
    T: Send + 'static,
    F: FnOnce() -> T + Send + 'static,
{
    let result = atp::run(move || Ok(f())).await;
    match result {
        Ok(r) => r,
        Err(BlockingError::Canceled) => panic!("Thread pool is gone"),
        Err(BlockingError::Error(())) => unreachable!(),
    }
}
