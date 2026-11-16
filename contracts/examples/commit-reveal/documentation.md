# Abstract

A commit-reveal round with KLV bonds. Each player locks a bond together with a hash of a secret value (the commit phase), then discloses the value and the salt (the reveal phase). Players who do not reveal in time forfeit their bond to the players who did.

The example is meant to show:

- deadlines based on block timestamps, with exact boundaries (`now < commit_deadline`, `commit_deadline <= now < reveal_deadline`, `now >= reveal_deadline`);
- bonds that make not revealing costly;
- a permissionless `finalize`: anyone can settle a round once the reveal deadline has passed;
- pull payments: `finalize` only credits balances, and each player withdraws with `claim`;
- an unambiguous commitment encoding that is bound to one round of one contract and to one player.

# Endpoints

- `createRound(commit_secs, reveal_secs, bond, max_players)`: opens a round and returns its id (ids start at 1).
- `commit(round_id, hash)`: payable, KLV only; joins a round with exactly the bond, once per address.
- `reveal(round_id, value, salt)`: discloses the value; the salt is exactly 32 bytes.
- `finalize(round_id)`: settles a round after the reveal deadline; revealers get their bond back plus an equal share of the forfeited bonds, the division remainder goes to the first revealer in commit order. If nobody revealed, all bonds are forfeit and credited to the contract owner. Finalize then removes the round's per-player storage (players, commitments, revealed values) and marks the round finalized.
- `claim()`: pays out everything credited to the caller.
- Views: `computeCommitment(round_id, player, salt, value)`, `getRound`, `getPlayers`, `getCommitment`, `getRevealed`, `getClaimable`. `getPlayers`, `getCommitment` and `getRevealed` return empty values once the round is finalized.

# Events

- `valueRevealed(round_id, player, value)`: emitted by every successful `reveal`; `round_id` and `player` are indexed topics, `value` is the data. Because `finalize` removes the stored values, these events are the lasting record of what each player revealed.
- `roundFinalized(round_id, revealer_count, forfeit_total)`: emitted by `finalize`; `round_id` and `revealer_count` are indexed topics, `forfeit_total` (the bonds of players who did not reveal, which is every bond when nobody revealed) is the data.

# Rules

| Rule | Value | Error |
| --- | --- | --- |
| Commit and reveal phase length | at least 60 seconds each | `phase too short` |
| Commit and reveal phase length | at most 30 days each, so a round cannot lock bonds indefinitely | `phase too long` |
| Deadline arithmetic | checked addition | `phase too long` |
| Bond | greater than 0 | `bond must be positive` |
| Players per round | 2 to 100 | `invalid max players` |
| Commit | only while `now < commit_deadline` | `commit phase over` |
| Commit payment | exactly the bond, in KLV | `wrong bond`; other tokens are rejected by the `#[payable("KLV")]` check with `function does not accept KDA payment` |
| Commit | one per address; round not full | `already committed`, `round full` |
| Reveal | only while `commit_deadline <= now < reveal_deadline` | `not in reveal phase` |
| Reveal | needs a commitment, once, non-empty value, matching hash | `no commitment`, `already revealed`, `empty value`, `hash mismatch` |
| Reveal | salt of exactly 32 bytes | argument decode error |
| Finalize | only once `now >= reveal_deadline`, once per round | `too early`, `already finalized` |
| Claim | needs a credited balance | `nothing to claim` |
| Commit, Reveal | the round is not finalized (defense in depth: timestamps already rule it out on chain) | `round finalized` |
| Any round call | the round must exist | `round not found` |

# Trust limits

The example is kept simple for teaching. Weigh these before reusing it:

- **Sybil discount.** Forfeited bonds go to the revealers, so a player who holds several seats in a round gets back part of the bond one of their own addresses forfeits. With two of three seats, withholding one reveal costs half a bond, not a whole one. If that matters, send forfeits to a sink no player controls (burn or a treasury).
- **Open rounds.** Anyone can create a round and anyone can commit to it. A party that fills every seat and never reveals blocks other players with `round full`; if that party is the contract owner, the no-reveal pot comes back to them at `finalize`. If that matters, restrict who may commit (an allowlist or the round creator's choice), or route a no-reveal pot somewhere other than the owner.
- **Upgradeable deployment.** The contract has an `#[upgrade]` endpoint. Deployed as upgradeable, the owner can replace the code and take every locked bond and every unclaimed balance: full custody of the funds, not just the no-reveal pot. Deploy it non-upgradeable for real use.
- **Censorship near the deadline.** A reveal only counts if a block includes it before `reveal_deadline`. A validator who also plays, or a group of colluding validators, could leave other players' reveals out of the last blocks and collect their forfeited bonds. The 60-second minimum is about 15 blocks; for rounds with real value, use a reveal phase of several minutes and reveal early, so that the reveal lands in a block proposed by someone else.
- **Check the round before committing.** Read `getRound` first: the deadlines, the bond and `max_players` are whatever the round's creator chose, within the limits above.

# Gas

`finalize` is the only path that releases bonds, so a full round must settle in one transaction. Measured on a local devnet running klever-go `develop`, finalizing a 100-player round (`MAX_PLAYERS`) with 99 revealers used 65,219,618 gas, about one sixth of the 400,000,000 gas a transaction may use, and the cost grows linearly with the number of players. The scenarios run with the dummy gas schedule, so they do not check this number: measure again if you raise `MAX_PLAYERS` or add work to `finalize`.

# Computing a commitment

The commitment is `keccak256` of the following bytes, in this order:

```
round_id   8 bytes, big-endian u64
contract   32 bytes, the contract address
player     32 bytes, the address that will call commit and reveal
salt       32 bytes
value      the rest (non-empty)
```

All fields except `value` have a fixed length and `value` comes last, so the preimage splits into fields in only one way. The round id and the contract address make a hash valid for one round of one contract only, and the player address means nobody else can reveal a copied hash.

Pick a random 32-byte salt and keep it secret until the reveal phase. The `computeCommitment` view returns the same hash, but it takes the secret as input: querying it through a node sends your salt and value to that node, and calling it as a transaction publishes them on-chain. For real use, compute the hash locally with the layout above and use the view only for testing.

# Running the tests

```
cargo test -p commit-reveal
```

The scenarios in `scenarios/` run on the Rust VM. The Go scenario tests need the `operator` binary on `PATH` and a feature flag:

```
cargo test -p commit-reveal --features klever-sc-scenario/run-go-tests
```

To build the contract, install the wasm target once with `sc-meta install wasm32`, then run `cargo run build` in the `meta/` folder.
