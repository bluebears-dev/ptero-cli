use std::sync::mpsc::Sender;
use rand::prelude::StdRng;
use rand::{Rng, RngCore, SeedableRng};

/// Status `enum` of the steganographic method.
/// Send to the observer in [`CommonMethodConfig`] during hiding/revealing.
#[derive(Debug, Copy, Clone)]
pub enum MethodProgressStatus {
    /// Informs about step progress (increment) - amount of written data into the cover.
    DataWritten(u64),
    /// Process has been completed.
    Finished,
}

/// Common configuration for all steganographic methods.
#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct CommonMethodConfig<'a> {
    /// Observer that recognizes [`MethodProgressStatus`].
    /// This can be used to track the progress of hiding/revealing.
    #[builder(setter(into, strip_option, name = "register"), default)]
    pub observer: Option<&'a Sender<MethodProgressStatus>>,
    /// Random number generator used by methods.
    /// By default populated with [`StdRng::from_entropy`].
    #[builder(setter(prefix = "with"), default = "Box::new(StdRng::from_entropy())")]
    pub rng: Box<dyn RngCore>,
}

impl<'a> CommonMethodConfig<'a> {
    /// Provides builder for safe configuration construction.
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// ```
    /// use ptero::method::config::{CommonMethodConfig, CommonMethodConfigBuilder};
    ///
    /// let default_config = CommonMethodConfig::builder().build();
    /// // Or by explicitly referencing builder
    /// let default_config = CommonMethodConfigBuilder::default().build();
    /// ```
    ///
    /// Provide custom props
    /// ```
    /// use std::sync::mpsc::channel;
    /// use ptero::method::config::{CommonMethodConfig, CommonMethodConfigBuilder, MethodProgressStatus};
    /// use rand::rngs::mock::StepRng;
    /// use rand::Rng;
    ///
    /// let (tx, rx) = channel::<MethodProgressStatus>();
    ///
    /// let mut config = CommonMethodConfig::builder()
    ///     .with_rng(Box::new(StepRng::new(2, 1)))
    ///     .register(&tx)
    ///     .build()
    ///     .unwrap();
    ///
    /// assert_eq!(config.rng.gen::<u32>(), 2)
    /// ```
    pub fn builder() -> CommonMethodConfigBuilder<'a> {
        CommonMethodConfigBuilder::default()
    }
}