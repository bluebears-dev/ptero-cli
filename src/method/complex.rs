pub mod eluv;
pub mod extended_line;

#[macro_export]
macro_rules! impl_complex_encoder {
    ($t:ident) => {
        impl crate::encoder::Encoder for $t {
            fn encode(
                &mut self,
                context: &mut crate::context::Context,
                data: &mut dyn Iterator<Item = crate::binary::Bit>,
                line: &mut String,
            ) -> Result<crate::encoder::EncoderResult, Box<dyn std::error::Error>> {
                let mut is_data_still_available = crate::encoder::EncoderResult::Success;
                for encoder in &mut self.methods {
                    if let crate::encoder::EncoderResult::NoDataLeft =
                        encoder.encode(context, data, line)?
                    {
                        is_data_still_available = crate::encoder::EncoderResult::NoDataLeft;
                        break;
                    }
                }
                Ok(is_data_still_available)
            }

            fn rate(&self) -> u32 {
                self.methods
                    .iter()
                    .fold(0, |acc, encoder| acc + encoder.rate())
            }
        }
    };
}

#[macro_export]
macro_rules! impl_complex_decoder {
    ($t:ident) => {
        impl crate::decoder::Decoder for $t {
            fn decode(
                &self,
                context: &crate::context::Context,
                line: &str,
            ) -> Result<Vec<crate::binary::Bit>, crate::context::ContextError> {
                let mut secret_data = Vec::default();
                for decoder in &self.methods {
                    let mut result = decoder.decode(context, line)?;
                    secret_data.append(&mut result);
                }
                Ok(secret_data)
            }
        }
    };
}
