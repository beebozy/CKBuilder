# CKB Learning Report — Week 7

## Introduction

This week shifted from cryptographic core development to building the **CLI wallet** that makes the ML-DSA-65 Dilithium lock script usable end-to-end. The goal was to take the wallet from "a lock script that verifies signatures" to "a tool that can generate addresses, check balances, and move funds on testnet."

The wallet MVP was built out in full: keygen, address derivation, balance querying, and transfer construction/signing, all wired into a CLI (`dillithium-wallet`). Address generation works cleanly end-to-end. Balance querying does not — and isolating *why* became the main technical work of the week, surfacing a clear split between the on-chain state (correct) and the off-chain SDK query logic (broken).

## Topics Covered

### 1. CLI Wallet Architecture for a Custom Lock Script

Built the wallet as a thin CLI layer over the existing SDK modules, rather than a monolithic tool.

**Key Learnings**
- A usable wallet needs four core operations: `keygen`, `address`, `balance`, `transfer` — each mapping to a distinct SDK responsibility (key management, address encode/decode, RPC/indexer queries, transaction building + signing).
- Splitting address encode/decode, the RPC/indexer client, raw transaction hashing, transfer building, and signing/sealing into separate modules made it possible to isolate the balance bug to a single layer instead of debugging the whole pipeline.
- Minimum cell capacity calculation has to be wallet-aware, not just contract-aware — 32-byte lock args push minimum capacity to ~73 CKB, which affects how much a transfer needs to reserve for change.
- CLI argument parsing needs explicit validation (`requireOption`) per command, since CKB endpoints, networks, and addresses are all easy to mistype or wrap onto a second shell line accidentally.

**Understanding Gained**

The wallet CLI is the first piece of tooling in this project that a non-developer-facing user would actually touch. Unlike the lock script or the WASM crypto core, bugs here are immediately visible as "the wallet is wrong" rather than "the test suite is wrong" — which raises the stakes for getting the RPC/indexer integration right, since a balance bug at this layer can look exactly like funds being missing, even when they're not.

### 2. End-to-End Address Generation and Validation

Verified that `keygen` → `address` produces a stable, correct CKB address across networks.

**Key Learnings**
- The same Dilithium public key produces a consistent lock script (`codeHash`, `hashType: "data2"`, `args`) regardless of which network flag (`devnet`/`testnet`) is passed — only the address prefix encoding changes, not the underlying lock.
- `hashType: "data2"` is the correct hash type for this contract's deployment, and it round-trips correctly through address encoding.
- A subtle shell-parsing pitfall: splitting `--network` and its value (`devnet`) across two lines in zsh without a trailing backslash causes the shell to treat `devnet` as a separate command (`zsh: command not found: devnet`), not a CLI argument error. This looks like a CLI bug but is actually a terminal usage issue.

**Understanding Gained**

Address generation being solid is what makes it possible to trust the *lock script* derivation in isolation, separate from anything network- or RPC-related. This was an important checkpoint: it confirmed the bug investigated later in the week is not a key-derivation or address-encoding problem, since the exact same lock script that produces a "0 balance" through the wallet CLI is provably funded on-chain.

### 3. Diagnosing the Balance Query Bug: On-Chain State vs. Off-Chain SDK Logic

This was the core debugging effort of the week. `dillithium-wallet balance` consistently reported `totalCapacity: "0"`, `liveCells: 0` for a wallet address — the question was whether the chain had no funds, or whether the SDK was failing to find funds that existed.

**Key Learnings**
- The diagnostic method: bypass the SDK entirely and query the indexer directly via raw `curl` against `get_cells`, using the *exact same* `code_hash`, `hash_type`, and `args` that the CLI's `address` command had just printed.
- The raw indexer call returned **3 live cells**, each with `capacity: 0xe8d4a51000` (1000 CKB), for the identical lock script the CLI reported as having zero balance.
- This is conclusive: the chain/indexer state is correct and funded. The bug is isolated to the SDK's query path — specifically `getBalanceSummary`, `getCells`, or `CkbRpcClient` — not to lock-script derivation, address encoding, or on-chain state.
- The bug class this points to is a **request-construction or response-parsing mismatch** inside the SDK's indexer client — e.g. malformed `script_type`/`filter` parameters, an incorrect cursor/pagination handling, or a response field name mismatch when parsing `get_cells` results into `liveCells`/`totalCapacity`.
- Reading `cli.ts` confirmed `runBalance` delegates directly to `getBalanceSummary(client, lock)` — the CLI itself isn't the problem; the bug is one layer deeper, inside the SDK's RPC/indexer client implementation.

**Understanding Gained**

This week reinforced a debugging principle that's specific to blockchain tooling: when a balance shows as zero, the chain is *not* the first suspect — the query layer is. A raw RPC call that bypasses every abstraction is the fastest way to split "no funds" from "funds exist, tooling can't see them." Confirming the cells exist before touching a single line of SDK code avoided wasting time "fixing" a transfer flow that was never actually broken — the bug is entirely upstream, in how the wallet *reads* state, not in transaction construction or signing.

### 4. Build Tooling and Workflow Friction

Several non-cryptographic issues surfaced while getting the wallet runnable, worth tracking even though they're not core wallet logic.

**Key Learnings**
- `make build CONTRACT=dillithium-lock` successfully produces the contract artifact at `build/release/dillithium-lock`, but the root Makefile then tries to build a non-existent `dillithium-lock-sim` target afterward and exits non-zero — a stale reference from earlier scaffolding, not a real build failure.
- Until that Makefile path is fixed, the reliable contract test sequence is manual: cross-compile to `riscv64imac-unknown-none-elf`, copy the binary into `build/release/`, then run `cargo test --package tests` directly.
- CLI commands must be run from the repo root, since the wallet reads `deployment/scripts.json` relative to the current working directory — running from a subdirectory silently breaks config loading.

**Understanding Gained**

None of these are wallet-logic bugs, but they're exactly the kind of friction that wastes debugging time if not separated out from real bugs early. Distinguishing "the Makefile has a stale step" from "the contract build failed" and "the CLI needs network args" from "the balance query is broken" kept this week's investigation focused on the one bug that actually mattered.

## Practical Development Work

### CLI Wallet — `dillithium-wallet`

**Activities Completed**
- Built the wallet MVP on top of the existing Dilithium SDK modules: address encode/decode, CKB RPC/indexer client, raw transaction hashing, transfer building, signing/sealing, and minimum capacity calculation.
- Implemented CLI commands: `keygen`, `address`, `balance`, `transfer`.
- Replaced template Rust contract tests with real Dilithium lock script coverage; expanded SDK tests for wallet behavior.
- Verified `npm test` and `cargo test --package tests` both pass.
- Generated a real wallet keypair and derived working CKB addresses on both `devnet` and `testnet` — confirmed identical lock script across networks, differing only in address prefix.
- Diagnosed the balance bug using a direct indexer `get_cells` call, confirming 3 funded live cells exist on-chain for the wallet's lock script while the CLI reports zero.

**Status**
| Command | Status |
|---|---|
| `keygen` | Working — generates and saves keypair |
| `address` | Working — correct lock script + address across networks |
| `balance` | Bug confirmed — reports 0 despite 3 funded live cells on-chain |
| `transfer` | ⏸ Blocked — depends on `getCells`, same code path implicated in the balance bug |

**Next Steps**
- Inspect `getBalanceSummary`, `getCells`, and `CkbRpcClient` source directly to find where the indexer response is being lost or misparsed.
- Compare the SDK's constructed `get_cells` request payload against the raw `curl` request that worked, field by field.
- Once `getCells` is fixed, re-test `transfer`, since it depends on the same client method to source input cells.
- Fix the root Makefile's stale `dillithium-lock-sim` build step.