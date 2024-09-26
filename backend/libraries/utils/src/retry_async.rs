use std::future::Future;
use std::time::Duration;
use anyhow::Result;
use tracing::error;

pub async fn retry_with_attempts<F, Fut, T, Out>(
    max_attempts: u8,
    delay_duration: Duration,
    f: F
)
    -> Result<()>
    where F: FnMut() -> Fut + 'static, Fut: std::future::Future<Output = Out>, Out: Into<Result<T>>
{
    fn recursive<F, Fut, T, Out>(mut f: F, attempt: u8, max_attempts: u8, delay_duration: Duration)
        where
            F: FnMut() -> Fut + 'static,
            Fut: std::future::Future<Output = Out>,
            Out: Into<Result<T>>
    {
        ic_cdk_timers::set_timer(delay_duration, move || {
            ic_cdk::spawn(async move {
                match f().await.into() {
                    Ok(_) => (),
                    Err(_) if attempt < max_attempts =>
                        recursive(f, attempt + 1, max_attempts, delay_duration),
                    Err(_) => {
                        error!("Failed to execute action after {} attempts", max_attempts);
                    }
                }
            });
        });
    }

    recursive(f, 0, max_attempts, delay_duration);

    Ok(())
}

pub async fn retry_async<F, Fut, T, E>(mut operation: F, retries: usize) -> Result<T, E>
    where F: FnMut() -> Fut, Fut: Future<Output = Result<T, E>>
{
    let mut attempt = 0;
    while attempt < retries {
        attempt += 1;
        match operation().await {
            Ok(result) => {
                return Ok(result);
            }
            Err(err) => {
                if attempt >= retries {
                    return Err(err);
                }
            }
        }
    }
    unreachable!() // The code should never reach this point.
}

#[cfg(test)]
mod tests {
    use std::{ cell::RefCell, rc::Rc };

    use super::*;

    #[tokio::test]
    async fn test_retry_async_works_correctly() {
        let iteration_count = Rc::new(RefCell::new(0));
        let result = retry_async(|| async {
            let iteration_count = Rc::clone(&iteration_count);
            *iteration_count.borrow_mut() += 1;
            let success = true;
            if success {
                Ok(1)
            } else {
                Err(0)
            }
        }, 3).await;
        assert_eq!(*iteration_count.borrow(), 1);
        assert_eq!(result, Ok(1));

        let iteration_count = Rc::new(RefCell::new(0));
        let result = retry_async(|| async {
            let iteration_count = Rc::clone(&iteration_count);
            *iteration_count.borrow_mut() += 1;

            let success = false;
            if success {
                Ok(1)
            } else {
                Err(0)
            }
        }, 3).await;

        assert_eq!(*iteration_count.borrow(), 3);
        assert_eq!(result, Err(0));

        let iteration_count = Rc::new(RefCell::new(0));
        let result = retry_async(|| async {
            let iteration_count = Rc::clone(&iteration_count);
            *iteration_count.borrow_mut() += 1;

            if *iteration_count.borrow() == 2 {
                return Ok(1);
            }

            let success = false;
            if success {
                Ok(2)
            } else {
                Err(0)
            }
        }, 3).await;

        assert_eq!(*iteration_count.borrow(), 2);
        assert_eq!(result, Ok(1));
    }
}
