
# CKB Learning Report — Week 6

## Introduction

This week focused on two interconnected objectives: building the off-chain developer tooling for the ML-DSA-65 Dilithium lock script implemented in Week 5, and studying the SSRI (Script-Sourced Rich Information) protocol as a framework for how on-chain scripts can expose structured information and callable interfaces to off-chain applications.

The development work involved compiling the Dilithium cryptographic core to WebAssembly, wrapping it in a TypeScript SDK, and validating the full off-chain signing pipeline through automated tests. The conceptual work involved understanding how SSRI addresses the gap between on-chain verification logic and off-chain application integration on CKB.

---

## Topics Covered

### 1. Off-Chain SDK Design for Custom Lock Scripts

Designed and implemented a TypeScript SDK to complement the on-chain Dilithium lock script.

**Key Learnings**

- A lock script alone is not usable without off-chain tooling that can generate keys, derive lock arguments, construct witnesses, and sign transactions.
- The off-chain SDK must mirror the on-chain witness parsing logic exactly — any serialization mismatch causes silent verification failures at the script level.
- The witness format used by the Dilithium lock (`[u32 pubkey_len | pubkey | u32 sig_len | sig]`, little-endian) must be reproduced byte-for-byte in the TypeScript layer.
- Lock arguments are derived by computing a Blake2b-256 hash of the public key using the CKB personalization string `b"ckb-default-hash"` — the same function used inside the on-chain contract.
- Separating concerns into distinct modules (crypto, hash, lock, witness, types) makes the SDK easier to test and maintain.

**Understanding Gained**

Building the SDK clarified that a CKB lock script is only one half of a complete system. The on-chain contract defines the verification rules, but the off-chain SDK is what enables those rules to be satisfied in practice. For novel cryptographic schemes like Dilithium, this off-chain layer is critical because no existing wallet tooling supports it — the SDK must provide everything from key generation to transaction assembly.

---

### 2. WebAssembly as a Bridge Between Rust Cryptography and TypeScript

Compiled the Dilithium cryptographic core from Rust to WebAssembly for use in the TypeScript SDK.

**Key Learnings**

- The `wasm32-unknown-unknown` target produces a bare WebAssembly module without any JavaScript glue, keeping the binary lean and portable.
- Raw WASM memory passing (using pointers via `alloc`/`dealloc` exports) is more lightweight than `wasm-bindgen` for this use case and avoids a JavaScript glue code dependency.
- Exported WASM functions (`generate_keypair_from_seed`, `sign_message_with_seed`, `verify_signature`, `hash_pubkey`) correspond directly to the cryptographic operations needed by the SDK.
- Using the same `fips204` Rust crate in both the on-chain RISC-V contract and the off-chain WASM module guarantees cryptographic consistency between the two environments.
- The compiled WASM binary is approximately 3MB — larger than typical WASM modules due to the complexity of lattice-based cryptography.

**Understanding Gained**

WebAssembly provides a practical path for bringing high-assurance cryptographic implementations into JavaScript environments without reimplementing algorithms from scratch. For post-quantum schemes where no mature JavaScript libraries exist, compiling a trusted Rust implementation to WASM is both the safest and most consistent approach. The same mathematical operations executing on CKB-VM (RISC-V) and in the browser/Node (WASM) eliminates the risk of implementation divergence between signing and verification.

---

### 3. SSRI — Script-Sourced Rich Information Protocol

Studied the SSRI protocol as a standardized interface layer for CKB smart contracts.

**Key Learnings**

- SSRI (Standard Smart Contract Runtime Interface) provides a standardized way to develop smart contracts that are compliant with the SSRI protocol, enabling better interoperability and a more consistent development experience across the CKB ecosystem.
- SSRI provides pre-defined public traits that receive first-class ecosystem support, utility functions for SSRI-VM syscalls and data handling, and procedural macros that simplify contract development with automatic SSRI method generation.
- SSRI allows a script to expose callable methods that off-chain applications can invoke, creating a structured API surface rather than requiring applications to reverse-engineer witness formats.
- The `ckb-ssri-std` crate provides the on-chain implementation layer, while a corresponding off-chain client handles method dispatch and result decoding.
- SSRI is particularly relevant for token standards — it defines pre-built public traits for UDT (User-Defined Token) implementations that receive first-class ecosystem support.

**Understanding Gained**

SSRI addresses a fundamental gap in CKB's scripting model. Without SSRI, off-chain applications must have out-of-band knowledge of a script's witness structure and behavior. With SSRI, the script itself becomes the source of truth for how it should be interacted with — embedding its interface definition directly on-chain. For the Dilithium lock script, SSRI integration would mean exposing methods like `getLockArgs`, `verifyWitness`, or `getPublicKeyLength` as callable interfaces, allowing wallets and dApps to query the script's own behavior without needing hardcoded knowledge of its internals.

---

### 4. Testing Strategy for Cryptographic SDKs

Designed and executed a test suite validating the full off-chain signing pipeline.

**Key Learnings**

- Unit tests should cover each layer independently: key generation correctness, lock args determinism, witness round-trip serialization, and signature agreement.
- A passing test suite against a placeholder binary is not sufficient — the real compiled WASM binary must be verified by inspecting its exports before trusting test results.
- WebAssembly module exports can be inspected at runtime using `WebAssembly.Module.exports()`, confirming that the expected cryptographic functions are present in the binary.
- The five core test cases for this SDK are: keypair size correctness, lock args determinism, witness round-trip, sign/verify agreement, and malformed witness rejection.

**Understanding Gained**

Cryptographic SDKs require a higher standard of testing than typical application code because failures are silent and potentially catastrophic — a malformed witness that passes unit tests but fails on-chain wastes transaction fees and may lock funds permanently. Verifying that tests run against the real compiled binary, not a stub, is a prerequisite for trusting the test results.

---

## Practical Development Work

### TypeScript SDK — `@ckb/dillithium-sdk`

Implemented and tested a complete off-chain SDK for the Dilithium lock script.

**Activities Completed**

- Compiled the `fips204` Dilithium implementation to WebAssembly targeting `wasm32-unknown-unknown`.
- Implemented a raw WASM memory interface in TypeScript with `alloc`/`dealloc` lifecycle management.
- Built `generateKeypair()`, `signTxHash()`, and `verifySignature()` as the primary SDK API.
- Implemented `computeLockArgs()` — Blake2b-256 hash of the public key using CKB personalization.
- Implemented `buildWitnessLock()` and `parseWitnessLock()` matching the on-chain witness format exactly.
- Verified WASM binary exports at runtime confirming all cryptographic functions are present.
- All 5 unit tests passing against the real compiled binary.

**Test Results**

| Test | Result | Duration |
|---|---|---|
| generateKeypair returns correctly sized keys |  Pass | 9.6ms |
| computeLockArgs is deterministic |  Pass | 2.3ms |
| buildWitnessLock round-trips through parseWitnessLock |  Pass | 2.5ms |
| signTxHash and verifySignature agree |  Pass | 3.9ms |
| buildWitnessLock rejects malformed lengths |  Pass | 0.4ms |

**Repository**

GitHub Repository: [ckb-pq-dillithium](https://github.com/beebozy/ckb-pq-dillithium-)

---

## Challenges Encountered

- Distinguishing between a real compiled WASM binary and a placeholder — the copy script succeeded silently in both cases, requiring explicit runtime export inspection to confirm binary integrity.
- Managing WASM linear memory manually in TypeScript — every `alloc` call requires a corresponding `dealloc` to avoid memory leaks, and pointer arithmetic must be precise for multi-argument functions like `sign_message_with_seed`.
- Understanding SSRI's method dispatch model — unlike traditional function calls, SSRI methods are invoked through a specialized VM syscall mechanism that requires the contract to handle method routing internally.

These challenges deepened understanding of:

- WebAssembly memory model and lifecycle management.
- The relationship between on-chain binary verification and off-chain tooling correctness.
- How protocol-level standards like SSRI reduce integration friction for novel script types.

---

## Key Takeaways

By the end of Week 6:

- Gained practical experience building a WebAssembly cryptographic module and integrating it into a TypeScript SDK.
- Developed a complete off-chain tooling layer for a custom CKB lock script, covering key generation, witness construction, and signature verification.
- Understood the SSRI protocol as a mechanism for scripts to expose structured interfaces to off-chain applications, and recognized its relevance to the Dilithium lock script as a future integration path.
- Established a reproducible test suite validating the full off-chain signing pipeline against the real compiled binary.
- Completed the second major deliverable toward the Nervos Foundation grant submission: the WASM crate and TypeScript SDK.