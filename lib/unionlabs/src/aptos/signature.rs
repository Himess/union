use macros::model;
use unionlabs_primitives::{FixedBytesError, H768};

#[model(proto(
    raw(protos::union::ibc::lightclients::movement::v1::AggregateSignature),
    into,
    from
))]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
pub struct AggregateSignature {
    pub validator_bitmask: ValidatorBitmask,
    pub sig: Option<H768>,
}

#[model]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
pub struct ValidatorBitmask {
    pub inner: Vec<u8>,
}

impl From<AggregateSignature>
    for protos::union::ibc::lightclients::movement::v1::AggregateSignature
{
    fn from(value: AggregateSignature) -> Self {
        Self {
            validator_bitmask: value.validator_bitmask.inner,
            sig: value.sig.map(Into::into).unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum TryFromAggregateSignatureError {
    #[error("invalid sig")]
    Sig(#[from] FixedBytesError),
}

impl TryFrom<protos::union::ibc::lightclients::movement::v1::AggregateSignature>
    for AggregateSignature
{
    type Error = TryFromAggregateSignatureError;

    fn try_from(
        value: protos::union::ibc::lightclients::movement::v1::AggregateSignature,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            validator_bitmask: ValidatorBitmask {
                inner: value.validator_bitmask,
            },
            sig: if value.sig.is_empty() {
                None
            } else {
                Some(value.sig.try_into()?)
            },
        })
    }
}
