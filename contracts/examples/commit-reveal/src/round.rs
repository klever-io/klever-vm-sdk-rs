use klever_sc::{api::ManagedTypeApi, types::BigUint};

use klever_sc::derive_imports::*;

/// Configuration and state of one commit-reveal round.
///
/// Deadlines are absolute block timestamps (seconds):
/// - commit phase: `now < commit_deadline`
/// - reveal phase: `commit_deadline <= now < reveal_deadline`
/// - finalize:     `now >= reveal_deadline`
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct RoundInfo<M: ManagedTypeApi> {
    pub commit_deadline: u64,
    pub reveal_deadline: u64,
    /// KLV every player locks when committing.
    pub bond: BigUint<M>,
    pub max_players: u32,
    pub finalized: bool,
}
