use unionlabs::primitives::H768;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
pub struct SyncAggregate {
    #[cfg_attr(feature = "serde", serde(with = "::serde_utils::hex_string"))]
    pub sync_committee_bits: Vec<u8>,
    pub sync_committee_signature: H768,
}

#[cfg(feature = "ssz")]
#[derive(Debug, Clone, PartialEq, ssz::Ssz)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(bound(serialize = "", deserialize = ""))
)]
pub struct SyncAggregateSsz<C: crate::SYNC_COMMITTEE_SIZE> {
    // TODO: Change debug print for this type in ssz::types
    // #[debug("BitVector({})", sync_committee_bits.iter().map(|b| if b { '1' } else { '0' }).collect::<String>())]
    pub sync_committee_bits: ssz::types::BitVector<C::SYNC_COMMITTEE_SIZE>,
    pub sync_committee_signature: H768,
}
