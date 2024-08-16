use ic_cdk_timers::TimerId;
use serde::{ Deserialize, Deserializer, Serialize, Serializer };
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::ops::Deref;
use std::rc::Rc;
use std::time::Duration;
use tracing::trace;
use utils::env::Environment;

use crate::timer_manager::TimerManager;

pub mod timer_manager;

pub struct TimerJobs<R> {
    jobs: BTreeMap<String, TimerManager<R>>,
}

type JobWrapper<J> = Rc<RefCell<Option<J>>>;

impl<R> TimerJobs<R> {
    pub fn iter(&self) -> impl Iterator<Item = &TimerManager<R>> {
        self.jobs.values()
    }

    pub fn len(&self) -> usize {
        self.jobs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.jobs.is_empty()
    }
}
pub trait Job: 'static {
    fn execute(self);
}
