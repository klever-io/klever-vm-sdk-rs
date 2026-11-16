#![no_std]

//! Commit-reveal round with KLV bonds and reveal deadlines.
//!
//! 1. Commit phase: each player locks `bond` KLV together with
//!    `keccak256(round_id ++ contract_address ++ player_address ++ salt ++ value)`
//!    (see `compute_commitment` for the exact byte layout).
//! 2. Reveal phase: each player discloses `value` and the 32-byte `salt`; the
//!    contract recomputes the hash. Binding the address means a copied
//!    commitment cannot be revealed by anyone else.
//! 3. After the reveal deadline anyone can finalize: revealers get their bond
//!    back plus an equal share of the bonds of players who did not reveal.
//! 4. Payouts are credited, then pulled with `claim`.

use klever_sc::imports::*;

mod round;

pub use round::RoundInfo;

/// Shortest allowed phase: 15 blocks of 4 seconds.
pub const MIN_PHASE_SECONDS: u64 = 60;
/// Longest allowed phase: 30 days, so a round cannot lock bonds indefinitely.
pub const MAX_PHASE_SECONDS: u64 = 60 * 60 * 24 * 30;
/// Upper bound on players, so `finalize` iterates a bounded set.
pub const MAX_PLAYERS: u32 = 100;
/// Salt length in bytes, fixed by the argument type: a fixed length keeps the
/// preimage unambiguous, and 32 random bytes stop others brute-forcing small
/// values.
pub const SALT_LEN: usize = 32;

#[klever_sc::contract]
pub trait CommitReveal {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    /// Opens a new round and returns its id (ids start at 1).
    /// The commit phase lasts `commit_secs` from now and the reveal phase
    /// `reveal_secs` after that; both must be between `MIN_PHASE_SECONDS` and
    /// `MAX_PHASE_SECONDS`.
    #[endpoint(createRound)]
    fn create_round(
        &self,
        commit_secs: u64,
        reveal_secs: u64,
        bond: BigUint,
        max_players: u32,
    ) -> u64 {
        require!(
            commit_secs >= MIN_PHASE_SECONDS && reveal_secs >= MIN_PHASE_SECONDS,
            "phase too short"
        );
        require!(
            commit_secs <= MAX_PHASE_SECONDS && reveal_secs <= MAX_PHASE_SECONDS,
            "phase too long"
        );
        require!(bond > 0u32, "bond must be positive");
        require!(
            (2..=MAX_PLAYERS).contains(&max_players),
            "invalid max players"
        );

        // Checked: the wasm build does not trap on overflow, and a wrapped
        // deadline could put the reveal deadline before the commit deadline.
        let now = self.blockchain().get_block_timestamp();
        let Some(commit_deadline) = now.checked_add(commit_secs) else {
            sc_panic!("phase too long")
        };
        let Some(reveal_deadline) = commit_deadline.checked_add(reveal_secs) else {
            sc_panic!("phase too long")
        };

        let round_id = self.last_round_id().update(|id| {
            *id += 1;
            *id
        });
        self.round(round_id).set(RoundInfo {
            commit_deadline,
            reveal_deadline,
            bond,
            max_players,
            finalized: false,
        });
        round_id
    }

    /// Joins a round during the commit phase (`now < commit_deadline`).
    /// Pay exactly the round's bond in KLV; one commitment per address.
    #[payable("KLV")]
    #[endpoint]
    fn commit(&self, round_id: u64, hash: ManagedByteArray<Self::Api, 32>) {
        let round = self.require_round(round_id);
        // Defense in depth: the timestamp checks below already rule this out on
        // a real chain, but a bond accepted after finalize could never be paid.
        require!(!round.finalized, "round finalized");
        let now = self.blockchain().get_block_timestamp();
        require!(now < round.commit_deadline, "commit phase over");
        require!(*self.call_value().klv_value() == round.bond, "wrong bond");

        let caller = self.blockchain().get_caller();
        let mut players = self.players(round_id);
        require!(!players.contains(&caller), "already committed");
        require!(players.len() < round.max_players as usize, "round full");

        self.commitment(round_id, &caller).set(&hash);
        players.insert(caller);
    }

    /// Discloses the committed value during the reveal phase
    /// (`commit_deadline <= now < reveal_deadline`). The value must be
    /// non-empty, the salt exactly `SALT_LEN` bytes (a wrong length fails
    /// argument decoding), and `compute_commitment(round_id, caller, salt,
    /// value)` must equal the commitment. Emits `valueRevealed`, the lasting
    /// record of the value: `finalize` deletes the stored copy.
    #[endpoint]
    fn reveal(
        &self,
        round_id: u64,
        value: ManagedBuffer,
        salt: ManagedByteArray<Self::Api, SALT_LEN>,
    ) {
        let round = self.require_round(round_id);
        require!(!round.finalized, "round finalized");
        let now = self.blockchain().get_block_timestamp();
        require!(
            round.commit_deadline <= now && now < round.reveal_deadline,
            "not in reveal phase"
        );

        let caller = self.blockchain().get_caller();
        let commitment = self.commitment(round_id, &caller);
        require!(!commitment.is_empty(), "no commitment");
        let revealed = self.revealed(round_id, &caller);
        require!(revealed.is_empty(), "already revealed");
        // An empty value would be stored as "not revealed" and forfeit the bond.
        require!(!value.is_empty(), "empty value");
        require!(
            self.compute_commitment(round_id, &caller, &salt, &value) == commitment.get(),
            "hash mismatch"
        );

        revealed.set(&value);
        self.value_revealed_event(round_id, &caller, &value);
    }

    /// Settles a round once `now >= reveal_deadline`; anyone may call it, once.
    ///
    /// Bonds of players who did not reveal are split equally among the
    /// revealers, who also get their own bond back. The division remainder
    /// goes to the first revealer in the players set's order, which is commit
    /// order (not reveal order). If nobody revealed, all bonds are forfeit
    /// and go to the contract owner. Amounts are credited to `claimable`.
    ///
    /// Trust limits of this split, by design of the example:
    /// - Sybil: a player with several addresses in one round gets back part
    ///   of the bond an address of theirs forfeits (with two of three seats,
    ///   withholding one reveal costs half a bond, not a whole one). Send
    ///   forfeits to a sink no player controls if that matters.
    /// - Anyone may create a round and anyone may fill it. A party that
    ///   commits from every seat and never reveals blocks other players; when
    ///   that party is the owner, the whole pot comes back to them. Restrict
    ///   who may commit, or route a no-reveal pot elsewhere, if that matters.
    /// - Upgrades: the contract has an `#[upgrade]` endpoint. Deployed as
    ///   upgradeable, the owner can replace the code and take every locked
    ///   bond and unclaimed balance, which is full custody of the funds, not
    ///   just the no-reveal pot. Deploy it non-upgradeable for real use.
    ///
    /// Per-player round storage (players, commitments, revealed values) is
    /// removed; the round info is kept and marked finalized. The revealed
    /// values stay readable in the `valueRevealed` events, and
    /// `roundFinalized` records the revealer count and the forfeited total.
    #[endpoint]
    fn finalize(&self, round_id: u64) {
        let mut round = self.require_round(round_id);
        let now = self.blockchain().get_block_timestamp();
        require!(now >= round.reveal_deadline, "too early");
        require!(!round.finalized, "already finalized");

        let mut players = self.players(round_id);
        let player_count = players.len();
        let revealer_count = players
            .iter()
            .filter(|player| self.has_revealed(round_id, player))
            .count();
        let forfeit_total = &round.bond * &BigUint::from(player_count - revealer_count);

        if revealer_count == 0 {
            // Every bond is forfeit. For an empty round this credits zero,
            // which stores nothing and leaves `claim` refusing.
            self.credit(&self.blockchain().get_owner_address(), &forfeit_total);
        } else {
            let revealers = BigUint::from(revealer_count);
            let share = &forfeit_total / &revealers;
            let mut remainder = &forfeit_total % &revealers;
            for player in players.iter() {
                if self.has_revealed(round_id, &player) {
                    let payout = &round.bond + &share + &remainder;
                    remainder = BigUint::zero();
                    self.credit(&player, &payout);
                }
            }
        }

        for player in players.iter() {
            self.commitment(round_id, &player).clear();
            self.revealed(round_id, &player).clear();
        }
        players.clear();

        round.finalized = true;
        self.round(round_id).set(&round);

        self.round_finalized_event(round_id, revealer_count as u32, &forfeit_total);
    }

    /// Pays out everything credited to the caller.
    /// The balance is zeroed before the transfer, so it can be claimed once.
    #[endpoint]
    fn claim(&self) {
        let caller = self.blockchain().get_caller();
        // `take` reads and clears in one step; a failing `require!` reverts both.
        let amount = self.claimable(&caller).take();
        require!(amount > 0u32, "nothing to claim");
        self.send().direct_klv(&caller, &amount);
    }

    /// The commitment a player must submit: `keccak256` of
    ///
    /// `round_id (8 bytes, big-endian) ++ contract address (32) ++ player (32)
    /// ++ salt (32) ++ value (rest)`
    ///
    /// Every field but `value` has a fixed length and `value` comes last, so a
    /// preimage splits into fields in exactly one way: a revealer cannot move
    /// bytes between `salt` and `value` to open the commitment as a different
    /// value. The round id and contract address make a hash valid for one
    /// round of one contract only, so it cannot be replayed in another round
    /// or on another deployment.
    ///
    /// Pure read, but it takes the secret as input: querying it sends `salt`
    /// and `value` in plaintext to the node that serves the query, and calling
    /// it as a transaction publishes them on-chain. Compute the hash locally
    /// with the layout above; use this view only against a local or trusted
    /// node, or to check test vectors.
    #[view(computeCommitment)]
    fn compute_commitment(
        &self,
        round_id: u64,
        player: &ManagedAddress,
        salt: &ManagedByteArray<Self::Api, SALT_LEN>,
        value: &ManagedBuffer,
    ) -> ManagedByteArray<Self::Api, 32> {
        let mut preimage = ManagedBuffer::new_from_bytes(&round_id.to_be_bytes());
        preimage.append(self.blockchain().get_sc_address().as_managed_buffer());
        preimage.append(player.as_managed_buffer());
        preimage.append(salt.as_managed_buffer());
        preimage.append(value);
        self.crypto().keccak256(&preimage)
    }

    fn require_round(&self, round_id: u64) -> RoundInfo<Self::Api> {
        let mapper = self.round(round_id);
        require!(!mapper.is_empty(), "round not found");
        mapper.get()
    }

    fn has_revealed(&self, round_id: u64, player: &ManagedAddress) -> bool {
        !self.revealed(round_id, player).is_empty()
    }

    fn credit(&self, to: &ManagedAddress, amount: &BigUint) {
        self.claimable(to).update(|balance| *balance += amount);
    }

    // events

    #[event("valueRevealed")]
    fn value_revealed_event(
        &self,
        #[indexed] round_id: u64,
        #[indexed] player: &ManagedAddress,
        value: &ManagedBuffer,
    );

    #[event("roundFinalized")]
    fn round_finalized_event(
        &self,
        #[indexed] round_id: u64,
        #[indexed] revealer_count: u32,
        forfeit_total: &BigUint,
    );

    // storage

    #[storage_mapper("lastRoundId")]
    fn last_round_id(&self) -> SingleValueMapper<u64>;

    #[view(getRound)]
    #[storage_mapper("round")]
    fn round(&self, round_id: u64) -> SingleValueMapper<RoundInfo<Self::Api>>;

    /// Players in commit order (the set is never shrunk before finalize).
    #[view(getPlayers)]
    #[storage_mapper("players")]
    fn players(&self, round_id: u64) -> UnorderedSetMapper<ManagedAddress>;

    #[view(getCommitment)]
    #[storage_mapper("commitment")]
    fn commitment(
        &self,
        round_id: u64,
        player: &ManagedAddress,
    ) -> SingleValueMapper<ManagedByteArray<Self::Api, 32>>;

    #[view(getRevealed)]
    #[storage_mapper("revealed")]
    fn revealed(&self, round_id: u64, player: &ManagedAddress) -> SingleValueMapper<ManagedBuffer>;

    /// KLV credited to an address and not yet claimed.
    #[view(getClaimable)]
    #[storage_mapper("claimable")]
    fn claimable(&self, player: &ManagedAddress) -> SingleValueMapper<BigUint>;
}
