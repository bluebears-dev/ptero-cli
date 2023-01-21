//! The observer pattern implementation for [`ptero_cli`] module.
//! # Examples
//!
//! Basic usage with custom listener:
//! ```
//! use ptero_common::observer::{EventNotifier, Observable, Observer};
//! use std::cell::RefCell;
//! use std::sync::{Arc, Weak};
//!
//! // Event that you would like to listen to
//! enum Event {
//!     Inc { step: usize },
//!     Dec { step: usize },
//!     Finish { msg: String }
//! }
//!
//! struct Counter {
//!     pub value: usize
//! }
//!
//! impl Observer<Event> for Counter {
//!     fn on_notify(&mut self, event: &Event) {
//!         match event {
//!             Event::Inc{ step } => { self.value += step; }
//!             Event::Dec{ step } => { self.value -= step; }
//!             Event::Finish{ msg } => {
//!                 println!("{} while counter is equal to {}", msg, self.value);
//!             }
//!         }
//!     }
//! }
//!
//! // Use [`EventNotifier`] to notify all subscribers to events
//! let mut event_notifier: EventNotifier<Event> = EventNotifier::default();
//! // You have to wrap observers into [`Arc`]
//! let counter_one = Arc::new(RefCell::new(Counter { value: 0 }));
//! let counter_two = Arc::new(RefCell::new(Counter { value: 4 }));
//!
//! event_notifier.subscribe(counter_one.clone());
//! event_notifier.subscribe(counter_two.clone());
//!
//! event_notifier.notify(&Event::Inc { step: 4 });
//! event_notifier.notify(&Event::Dec { step: 2 });
//! event_notifier.notify(&Event::Finish { msg: "Example message".to_string() });
//!
//! assert_eq!(counter_one.borrow().value, 2);
//! assert_eq!(counter_two.borrow().value, 6);
//! ```
//!
use std::cell::RefCell;
use std::sync::{Arc, Weak};

use log::warn;

pub trait Observer<Ev> {
    fn on_notify(&mut self, event: &Ev);
}

pub trait Observable {
    type Event;

    fn subscribe(&mut self, listener: Arc<RefCell<dyn Observer<Self::Event>>>);
}

#[derive(Clone)]
pub struct EventNotifier<Ev> {
    subscribers: Vec<Weak<RefCell<dyn Observer<Ev>>>>,
}

impl<Ev> EventNotifier<Ev> {
    pub fn new() -> EventNotifier<Ev> {
        EventNotifier {
            subscribers: Vec::new(),
        }
    }

    /// Sends an event to all valid subscribers.
    ///
    /// If [`Arc`] reference is not valid, proceeds to clean-up invalid subscribers.
    pub fn notify(&mut self, event: &Ev) {
        let mut cleanup_needed = false;

        for sub in self.subscribers.iter() {
            if let Some(subscriber_arc) = sub.upgrade() {
                let mut listener = subscriber_arc.borrow_mut();
                listener.on_notify(event);
            } else {
                warn!("Stale subscriber detected, clean-up needed");
                cleanup_needed = true;
            }
        }
        if cleanup_needed {
            self.cleanup_subscribers();
        }
    }

    pub fn count_subscribers(&self) -> usize {
        self.subscribers.len()
    }

    fn cleanup_subscribers(&mut self) {
        self.subscribers.retain(|weak_sub| {
            let reference = weak_sub.clone().upgrade();
            !matches!(reference, None)
        });
    }
}

impl<Ev> Default for EventNotifier<Ev> {
    fn default() -> Self {
        EventNotifier::new()
    }
}

impl<Ev> Observable for EventNotifier<Ev> {
    type Event = Ev;

    /// Adds new subscriber
    fn subscribe(&mut self, listener: Arc<RefCell<dyn Observer<Self::Event>>>) {
        self.subscribers.push(Arc::downgrade(&listener));
    }
}
