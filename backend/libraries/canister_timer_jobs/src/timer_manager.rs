use std::{ fmt::Debug, time::Duration };
use ic_cdk_timers::TimerId;
use types::TimestampMillis;
use crate::Environment;

pub struct TimerManager<R> {
    job_function: fn() -> R,
    function_name: String, // Store name just in case
    timer_id: Option<TimerId>,
    interval: Duration,
    max_attempts: u32,
    retry_delay_duration: Duration,
    last_run: Option<TimestampMillis>,
}

impl<R> Debug for TimerManager<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TimerManager")
            .field("function_name", &self.function_name) // Include function name in debug output
            .field("timer_id", &self.timer_id)
            .field("interval", &self.interval)
            .field("last_run", &self.last_run)
            .finish()
    }
}

use std::any::type_name;

impl<R> TimerManager<R> {
    pub fn new(
        job_function: fn() -> R,
        interval_secs: u64,
        max_attempts: Option<u32>,
        retry_delay_duration: Option<Duration>
    ) -> Self {
        let function_name = type_name::<fn() -> R>().to_string();
        Self {
            job_function,
            function_name,
            timer_id: None,
            interval: Duration::from_secs(interval_secs),
            max_attempts: max_attempts.unwrap_or(1),
            retry_delay_duration: retry_delay_duration.unwrap_or_default(),
            last_run: None,
        }
    }

    pub fn start_timer(&mut self, env: &dyn Environment) {
        let interval = self.interval;
        let job_function = self.job_function;
        let max_attempts = self.max_attempts;
        let retry_delay_duration = self.retry_delay_duration;

        self.timer_id = Some(
            ic_cdk_timers::set_timer_interval(
                interval,
                move || {
                    // job_function.run(max_attempts, retry_delay_duration);
                }
            )
        );

        self.last_run = Some(env.now());
    }

    pub fn cancel_timer(&mut self) {
        if let Some(timer_id) = self.timer_id.take() {
            ic_cdk_timers::clear_timer(timer_id);
        }
    }

    pub fn get_function_name(&self) -> &str {
        &self.function_name
    }
}

// Example function
async fn example_action() -> Result<(), String> {
    println!("Action executed");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::env::CanisterEnv;

    #[test]
    fn testing() {
        let env = CanisterEnv::new(true);
        let mut timer = TimerManager::new(example_action, 2, None, None);

        // timer.start_timer(&env);

        println!("Timer: {:?}", timer);

        timer.cancel_timer();
    }
}

// Helper function for retry logic with attempts
use tracing::error;
pub async fn retry_with_attempts<F, Fut>(max_attempts: u32, _delay_duration: Duration, mut f: F)
    where F: FnMut() -> Fut, Fut: std::future::Future<Output = Result<(), String>>
{
    for attempt in 1..=max_attempts {
        match f().await {
            Ok(_) => {
                break;
            } // If successful, break out of the loop
            Err(err) => {
                error!("Attempt {}: Error - {:?}", attempt, err);
                if attempt == max_attempts {
                    error!(
                        "Failed to execute the action after {} attempts: {:?}",
                        max_attempts,
                        err
                    );
                }
            }
        }
    }
}

pub trait SyncJob {
    fn run(self, max_attempts: u32, retry_delay_duration: Duration);
}

pub trait AsyncJob {
    fn run(self, max_attempts: u32, retry_delay_duration: Duration);
}

// Implement SyncJob for functions
impl<F> SyncJob for F where F: Fn() + 'static {
    fn run(self, max_attempts: u32, retry_delay_duration: Duration) {
        self();
    }
}

impl<F, Fut> AsyncJob
    for F
    where F: Fn() -> Fut + 'static, Fut: std::future::Future<Output = Result<(), String>> + 'static
{
    fn run(self, max_attempts: u32, retry_delay_duration: Duration) {
        // Clone `self` into the closure so that it is owned by the closure.
        let f = self;

        ic_cdk::spawn(async move {
            let _ = retry_with_attempts(max_attempts, retry_delay_duration, || async {
                f().await
            }).await;
        });
    }
}
