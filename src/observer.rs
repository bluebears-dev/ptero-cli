use std::collections::HashMap;

use snafu::Snafu;

pub struct EventNotifier<'a, E> {
    subscribers: HashMap<&'a str, Box<dyn Fn(&E)>>,
}

impl<'a, E> Default for EventNotifier<'a, E> {
    fn default() -> Self {
        EventNotifier::new()
    }
}

impl<'a, E> EventNotifier<'a, E> {
    pub fn new() -> EventNotifier<'a, E> {
        EventNotifier {
            subscribers: HashMap::new(),
        }
    }

    pub fn register<F>(&mut self, name: &'a str, callback: F) -> Result<(), EventNotifierError>
    where
        F: 'static + Fn(&E),
    {
        if self.subscribers.contains_key(name) {
            self.subscribers.insert(name, Box::new(callback));
            Ok(())
        } else {
            Err(EventNotifierError::KeyAlreadyPresent {
                key: name.to_string()
            })
        }
    }

    pub fn unregister(&mut self, name: &'a str) {
        self.subscribers.remove(name);
    }

    pub fn notify(&self, event: E) {
        for callback in self.subscribers.values() {
            callback(&event);
        }
    }
}

#[derive(Debug, PartialEq, Snafu)]
pub enum EventNotifierError {
    #[snafu(display("Cannot register under '{}' as this name is already occupied", key))]
    KeyAlreadyPresent { key: String },
}
