use crate::engine::types::{Event, Subscriber};

use std::collections::VecDeque;
pub struct Bus {
    queue: VecDeque<Event>, //Queue of messages
    subscribers: Vec<Subscriber>, //Will be dispatched
}

impl Bus {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            subscribers: Vec::new(),
        }
    }

    pub fn subscribe<F>(&mut self, callback: F)
    where
        F: Fn(Event) + Send + Sync + 'static,
    {
        self.subscribers.push(Box::new(callback));
    }

    pub fn dispatch(&mut self) {
        while let Some(event) = self.queue.pop_front() {
            for sub in self.subscribers.iter() {
                sub(event.clone());
            }
        }
    }

    pub fn post(&mut self, event: Event) {
        self.queue.push_back(event);
    }
}