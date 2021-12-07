use bitvec::prelude::*;
use bitvec::slice::Iter;

/// Possible results of data encoding
#[derive(Debug, Clone)]
pub enum MethodResult {
    Success,
    NoDataLeft,
}

/// Status `enum` of the steganographic method.
/// Send to the observer in [`CommonMethodConfig`] during hiding/revealing.
#[derive(Debug, Copy, Clone)]
pub enum MethodProgressStatus {
    /// Informs about step progress (increment) - amount of written data into the cover.
    DataWritten(u64),
    /// Process has been completed.
    Finished,
}

pub trait SteganographyMethod<Cover, Err> {
    type ConcealedOutput;

    fn try_conceal<Order, Type>(
        &mut self,
        cover: Cover,
        data: &mut Iter<Order, Type>,
    ) -> Result<Self::ConcealedOutput, Err>
        where
            Order: BitOrder,
            Type: BitStore;

    fn try_reveal<Order, Type>(&mut self, cover: Cover) -> Result<BitVec<Order, Type>, Err>
        where
            Order: BitOrder,
            Type: BitStore;
}