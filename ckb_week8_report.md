# CKB Learning Report — Week 8

## Introduction

This week closed out the bug that defined Week 7's investigation: the balance query returning zero despite confirmed funded cells on-chain. With `getCells`/`getBalanceSummary` now fixed, `transfer` — which was blocked on the same code path — was unblocked and verified end-to-end. The second half of the week shifted focus toward the Fiber Network, going through the official documentation and beginning the process of running a Fiber node locally as a first step toward understanding how it could integrate with or sit alongside the Dilithium wallet project.

## Topics Covered

### 1. Resolving the Balance Query Bug

Picked up directly from Week 7's diagnosis: chain state was confirmed correct (3 funded live cells via raw `curl` to `get_cells`), so the bug was isolated to the SDK's indexer client rather than lock-script derivation or on-chain state.

**Key Learnings**
- The root cause was in how the SDK constructed the indexer request — comparing the SDK's `get_cells` payload field-by-field against the working raw `curl` request surfaced the mismatch, exactly as planned at the end of Week 7.
- Pagination/cursor handling was a key suspect class identified last week, and tracing through `CkbRpcClient` confirmed the request-construction layer (not response parsing) was where the live cells were being silently dropped.
- Once corrected, `getBalanceSummary` returned the same 3 cells and total capacity that the raw indexer call had shown all along — confirming the fix targeted the actual bug rather than a coincidentally-related symptom.

**Understanding Gained**

The debugging principle from Week 7 — bypass the SDK with a raw RPC call to split "no funds" from "funds exist, tooling can't see them" — paid off directly this week. Having already proven the chain side was correct meant all debugging effort this week could go into the SDK's request-construction code with no ambiguity about where the bug lived, instead of re-litigating whether the chain or the client was at fault.

### 2. Unblocking and Verifying `transfer`

`transfer` had been blocked since it depends on `getCells` to source input cells — the same method implicated in the balance bug.

**Key Learnings**
- With `getCells` returning correct live cells, transfer construction (input selection, change output, minimum capacity handling) could finally be exercised against real funded cells on testnet rather than failing before reaching that logic at all.
- Re-testing transfer surfaced no *new* bugs — the transaction-building and signing logic from earlier weeks held up once it was actually given correct input data, reinforcing Week 7's conclusion that the transfer flow itself was never broken, only starved of correct cell data.
- A full `keygen` → `address` → `balance` → `transfer` → confirmation loop now runs cleanly against testnet.

**Understanding Gained**

This is the first week the wallet has worked end-to-end as a complete tool rather than as a set of individually-tested commands. It validates the earlier architectural choice (Week 7) to split the SDK into separate modules for address handling, RPC/indexer queries, and transaction building — the bug stayed contained to one module and fixing it there was sufficient to make the whole pipeline work, with no knock-on fixes needed elsewhere.

### 3. Fiber Network Documentation Review

Went through the official Fiber documentation (fiber.world/docs) to understand the network's architecture and how it relates to CKB.

**Key Learnings**
- Fiber is CKB's Lightning-Network-style payment channel layer, built for fast, low-fee off-chain transfers that settle back to CKB's on-chain cells when channels open/close.
- Conceptually, Fiber nodes manage channel state and route payments off-chain, only touching the CKB chain for channel funding, settlement, and disputes — a different trust and architecture model than the wallet's direct on-chain transfer flow built so far.
- Understanding Fiber's relationship to a custom lock script project like this one raises an open design question: whether a Dilithium-secured account could eventually act as a channel participant, and what constraints post-quantum signatures would add to channel state updates (e.g. signature size affecting on-chain settlement cost, similar to the witness-size cost concern already tracked for plain transfers).

**Understanding Gained**

Fiber sits at a different layer of the stack than anything built so far — it's not a replacement for the wallet's RPC/indexer/transfer logic, but a payment channel system that would eventually sit on top of an account model like this one. Reading the docs before attempting to run a node was useful for separating "what Fiber is for" from "how to get a node running," since the two are easy to conflate when reading setup instructions in isolation.

### 4. Attempting to Run a Fiber Node

Began working through the node setup instructions from the documentation as a hands-on next step.

**Key Learnings**
- [Add specifics here once the node is running or once a blocker is hit — e.g. which network/config the node is targeting (devnet/testnet), what dependencies or binaries were required, and any errors encountered during setup.]
- This is in progress; no node is fully running and synced yet as of end of week.


## Practical Development Work

### CLI Wallet — `dillithium-wallet`

**Activities Completed**
- Identified and fixed the root cause of the balance query bug inside the SDK's indexer client (request-construction mismatch, building on Week 7's diagnosis).
- Re-verified `getBalanceSummary` and `getCells` against testnet, confirming correct results matching the raw indexer call.
- Unblocked and tested `transfer`, running a full `keygen` → `address` → `balance` → `transfer` loop successfully on testnet.

**Status**

| Command | Status |
|---|---|
| `keygen` | Working — generates and saves keypair |
| `address` | Working — correct lock script + address across networks |
| `balance` | **Fixed** — correctly reflects funded live cells |
| `transfer` | **Working** — verified end-to-end on testnet |

### Fiber Network Exploration

**Activities Completed**
- Reviewed Fiber documentation (fiber.world/docs) covering network architecture and payment channel model.
- Began local Fiber node setup following documentation instructions.

**Status**

| Task | Status |
|---|---|
| Fiber docs review | Complete |
| Local Fiber node setup | In progress |

**Next Steps**
- Finish getting a Fiber node running locally and confirm it syncs/connects correctly.

- Consider whether the next wallet milestone should be UDT support, multisig design, or Fiber integration, now that the core transfer flow is fully functional.