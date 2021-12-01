use std::sync::mpsc::Sender;
use rand::RngCore;
use crate::method::config::{CommonMethodConfig, CommonMethodConfigBuilder, MethodProgressStatus};

pub enum ExtendedLineMethodVariant {
    V1,
    V2,
    V3,
}

pub struct ExtendedLineMethod<'a> {
    variant: ExtendedLineMethodVariant,
    config: CommonMethodConfig<'a>,
}

impl<'a> ExtendedLineMethod<'a> {
    pub fn builder() -> ExtendedLineMethodBuilder<'a> {
        ExtendedLineMethodBuilder::default()
    }
}

pub struct ExtendedLineMethodBuilder<'a> {
    variant: ExtendedLineMethodVariant,
    config_builder: CommonMethodConfigBuilder<'a>,
}

impl<'a> Default for ExtendedLineMethodBuilder<'a> {
    fn default() -> Self {
        ExtendedLineMethodBuilder {
            variant: ExtendedLineMethodVariant::V1,
            config_builder: CommonMethodConfig::builder()
        }
    }
}

impl<'a> ExtendedLineMethodBuilder<'a> {
    /// Set custom RNG for method.
    pub fn with_rng(mut self, rng: Box<dyn RngCore>) -> Self {
        self.config_builder = self.config_builder.with_rng(rng);
        self
    }

    /// Register progress status pipe
    pub fn register(mut self, observer: &'a Sender<MethodProgressStatus>) -> Self {
        self.config_builder = self.config_builder.register(observer);
        self
    }

    /// Set variant of the method
    pub fn with_variant(mut self, variant: ExtendedLineMethodVariant) -> Self  {
        self.variant = variant;
        self
    }

    /// Constructs the method
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use std::sync::mpsc::channel;
    /// use rand::rngs::mock::StepRng;
    /// use ptero::method::extended_line_method::{ExtendedLineMethod, ExtendedLineMethodVariant};
    /// use ptero::method::config::MethodProgressStatus;
    ///
    /// let (tx, rx) = channel::<MethodProgressStatus>();
    ///
    /// let method = ExtendedLineMethod::builder()
    ///     .with_rng(Box::new(StepRng::new(0, 1)))
    ///     .register(&tx)
    ///     .with_variant(ExtendedLineMethodVariant::V2)
    ///     .build();
    /// ```
    pub fn build(self) -> ExtendedLineMethod<'a> {
        ExtendedLineMethod {
            variant: self.variant,
            config: self.config_builder.build().unwrap()
        }
    }
}
