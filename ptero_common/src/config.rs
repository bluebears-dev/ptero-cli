use std::cell::RefCell;
use std::rc::{Rc, Weak};

use rand::RngCore;

use crate::method::MethodProgressStatus;
use crate::observer::EventNotifier;

impl CommonMethodConfigBuilder {
    pub fn with_rng<T>(mut self, rng: T) -> Self where T: RngCore + 'static {
        self.rng = Some(Box::new(rng));
        self
    }
}

/// Common configuration for all steganographic methods.
#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct CommonMethodConfig {
    /// Observer that recognizes [`MethodProgressStatus`].
    /// This can be used to track the progress of hiding/revealing.
    #[builder(setter(into, prefix = "with"), default)]
    pub notifier: EventNotifier<MethodProgressStatus>,
    /// Random number generator used by methods.
    /// By default populated with [`StdRng::from_entropy`].
    #[builder(private)]
    pub rng: Box<dyn RngCore>
}

impl CommonMethodConfig {
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
    /// use ptero_common::config::{CommonMethodConfig, CommonMethodConfigBuilder};
    ///
    /// let rng = StdRng::from_entropy();
    /// let default_config = CommonMethodConfig::builder()
    ///     .with_rng(rng)
    ///     .build();
    /// // Or by explicitly referencing builder
    /// let rng = StdRng::from_entropy();
    /// let default_config = CommonMethodConfigBuilder::default()
    ///     .with_rng(rng)
    ///     .build();
    /// ```
    ///
    /// Provide custom props
    /// ```
    /// use std::borrow::BorrowMut;
    /// use std::cell::RefCell;
    /// use std::rc::Rc;
    /// use rand::rngs::mock::StepRng;
    /// use rand::{Rng, RngCore};
    /// use ptero_common::config::CommonMethodConfig;
    ///
    /// let rng = StepRng::new(2, 1);
    ///
    /// let mut config = CommonMethodConfig::builder()
    ///     .with_rng(rng)
    ///     .build()
    ///     .unwrap();
    ///
    /// let ref_rng = &mut config.rng;
    /// assert_eq!(ref_rng.gen::<u32>(), 2)
    /// ```
    pub fn builder() -> CommonMethodConfigBuilder {
        CommonMethodConfigBuilder::default()
    }
}