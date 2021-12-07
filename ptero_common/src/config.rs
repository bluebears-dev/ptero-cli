use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::sync::mpsc::Sender;

use rand::{Rng, RngCore, SeedableRng};
use rand::prelude::StdRng;

use crate::method::MethodProgressStatus;
use crate::observer::{EventNotifier, EventNotifierError};

/// Common configuration for all steganographic methods.
#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct CommonMethodConfig<'a> {
    /// Observer that recognizes [`MethodProgressStatus`].
    /// This can be used to track the progress of hiding/revealing.
    #[builder(private, setter(into), default)]
    pub notifier: EventNotifier<'a, MethodProgressStatus>,
    /// Random number generator used by methods.
    /// By default populated with [`StdRng::from_entropy`].
    #[builder(private, setter())]
    pub rng: Weak<RefCell<dyn RngCore>>
}

impl<'a> CommonMethodConfigBuilder<'a> {
    pub fn register<F>(self, name: &'a str, listener: F) -> Result<(), EventNotifierError>
    where
        F: 'static + Fn(&MethodProgressStatus)
    {
        self.notifier.map(|mut notifier| {
            notifier.register(name, listener)
        }).unwrap()
    }

    pub fn with_rng(mut self, rng: &Rc<RefCell<dyn RngCore>>) -> Self {
        self.rng = Some(Rc::downgrade(rng));
        self
    }
}

impl<'a> CommonMethodConfig<'a> {
    /// Provides builder for safe configuration construction.
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// ```
    /// use std::cell::RefCell;
    /// use std::rc::Rc;
    /// use rand::{RngCore, SeedableRng};
    /// use rand::rngs::StdRng;
    /// use ptero::method::config::{CommonMethodConfig, CommonMethodConfigBuilder};
    ///
    /// let rng: Rc<RefCell<dyn RngCore>> = Rc::new(RefCell::new(StdRng::from_entropy()));
    /// let default_config = CommonMethodConfig::builder()
    ///     .with_rng(&rng)
    ///     .build();
    /// // Or by explicitly referencing builder
    /// let default_config = CommonMethodConfigBuilder::default()
    ///     .with_rng(&rng)
    ///     .build();
    /// ```
    ///
    /// Provide custom props
    /// ```
    /// use std::borrow::BorrowMut;
    /// use std::cell::RefCell;
    /// use std::rc::Rc;
    /// use std::sync::mpsc::channel;
    /// use rand::rngs::mock::StepRng;
    /// use rand::{Rng, RngCore};
    /// use ptero_common::config::{MethodProgressStatus, CommonMethodConfig};
    ///
    /// let (tx, rx) = channel::<MethodProgressStatus>();
    /// let rng: Rc<RefCell<dyn RngCore>> = Rc::new(RefCell::new(StepRng::new(2, 1)));
    ///
    /// let mut config = CommonMethodConfig::builder()
    ///     .with_rng(&rng)
    ///     .register(&tx)
    ///     .build()
    ///     .unwrap();
    ///
    /// let mut ref_rng = &*config.rng.upgrade().unwrap();
    /// let mut borrowed_rng = ref_rng.borrow_mut();
    /// assert_eq!(borrowed_rng.gen::<u32>(), 2)
    /// ```
    pub fn builder() -> CommonMethodConfigBuilder<'a> {
        CommonMethodConfigBuilder::default()
    }
}