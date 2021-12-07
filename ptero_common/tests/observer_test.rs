use std::cell::RefCell;
use std::sync::{Arc, Weak};

use ptero_common::observer::{EventNotifier, Observable, Observer};

// Event that you would like to listen to
enum Event {
    Inc { step: usize },
    Dec { step: usize },
    Finish { msg: String }
}

struct Counter {
    pub value: usize
}

impl Observer<Event> for Counter {
    fn on_notify(&mut self, event: &Event) {
        match event {
            Event::Inc{ step } => { self.value += step; }
            Event::Dec{ step } => { self.value -= step; }
            Event::Finish{ msg } => {
                println!("{} while counter is equal to {}", msg, self.value);
            }
        }
    }
}

#[test]
fn properly_cleans_up_when_invalid_reference_occurs() {
    let mut event_notifier: EventNotifier<Event> = EventNotifier::default();
    let counter_one = Arc::new(RefCell::new(Counter { value: 0 }));
    let counter_two = Arc::new(RefCell::new(Counter { value: 4 }));

    event_notifier.subscribe(counter_one.clone());
    event_notifier.subscribe(counter_two.clone());

    assert_eq!(event_notifier.count_subscribers(), 2);

    event_notifier.notify(&Event::Inc { step: 4 });
    event_notifier.notify(&Event::Dec { step: 2 });

    assert_eq!(counter_one.borrow().value, 2);
    assert_eq!(counter_two.borrow().value, 6);
    assert_eq!(event_notifier.count_subscribers(), 2);

    drop(counter_two);

    event_notifier.notify(&Event::Finish { msg: "Example message".to_string() });

    assert_eq!(event_notifier.count_subscribers(), 1);
}



