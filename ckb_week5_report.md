# CKB Learning Report — Week 5

## Introduction

This week focused on implementing a post-quantum signature verification lock script on CKB using **ML-DSA-65 (Dilithium)**, one of the digital signature schemes selected by NIST's Post-Quantum Cryptography standardization process. The objective was to understand how alternative cryptographic primitives can be integrated into the CKB scripting model and to gain practical experience deploying a custom lock script to the Nervos testnet.

The work involved building a complete Dilithium-based lock script in Rust, compiling it to RISC-V for CKB-VM, handling witness parsing, verifying public key ownership, and validating post-quantum signatures during transaction execution.

---

## Topics Covered

### 1. Post-Quantum Cryptography and ML-DSA

Studied the fundamentals of post-quantum cryptography and the role of ML-DSA (Dilithium) as a digital signature algorithm.

#### Key Learnings

* Post-quantum cryptography aims to remain secure against attacks from large-scale quantum computers.
* ML-DSA (formerly known as Dilithium) is a lattice-based digital signature scheme standardized by NIST.
* Unlike ECDSA and Schnorr signatures, ML-DSA relies on the hardness of lattice problems rather than discrete logarithms.
* ML-DSA-65 provides a balance between security and performance suitable for high-security applications.
* Post-quantum signatures typically require significantly larger public keys and signatures compared to classical cryptography.
* Signature verification remains deterministic and well-suited for blockchain validation.

#### Understanding Gained

Quantum-resistant cryptography introduces new trade-offs. While ML-DSA offers stronger long-term security assumptions against quantum adversaries, it comes with larger cryptographic artifacts and increased execution complexity. These trade-offs become particularly important in blockchain environments where storage, bandwidth, and execution costs directly affect usability.

---

### 2. Implementing a Custom Dilithium Lock Script

Developed a custom CKB lock script that validates ML-DSA-65 signatures.

#### Key Learnings

* Lock scripts can support any signature scheme as long as verification logic can execute within CKB-VM.
* The script stores a Blake2b hash of the public key inside the script arguments.
* During transaction verification:

  * The public key is provided through the witness.
  * The witness also contains the ML-DSA signature.
  * The script verifies that the provided public key hashes to the value stored in script arguments.
  * The signature is verified against the transaction hash.
* Witness data was manually parsed into public key and signature components.
* The `fips204` Rust implementation was used for ML-DSA verification.

#### Understanding Gained

The flexibility of the CKB scripting model allows entirely new cryptographic systems to be introduced without protocol-level changes. A lock script is fundamentally a verification program, meaning any cryptographic primitive that can be efficiently verified inside CKB-VM can be used to control ownership of assets.

---

### 3. Working with CKB Syscalls

Applied syscall concepts from previous weeks to build the verification flow.

#### Key Learnings

* `load_script()` was used to retrieve lock script arguments.
* `load_tx_hash()` was used to obtain the transaction hash being verified.
* `load_witness_args()` was used to retrieve witness data from the current script group.
* `Source::GroupInput` enabled access to witnesses associated with inputs using the same lock script.
* Script arguments were validated before cryptographic verification to prevent malformed inputs.

#### Understanding Gained

Building a custom signature scheme reinforced the importance of syscalls as the bridge between on-chain verification code and transaction data. The entire verification process depends on correctly loading transaction context and interpreting witness structures.

---

### 4. Script Deployment and Execution Costs

Investigated the deployment characteristics of post-quantum cryptography on CKB.

#### Key Learnings

* The compiled ML-DSA lock script occupied approximately **93 KB** on-chain.
* The large binary size is primarily due to the complexity of lattice-based cryptographic operations and supporting libraries.
* Larger script binaries increase deployment costs because the code must be stored permanently in cells.
* Compared to traditional secp256k1 lock implementations, post-quantum implementations introduce substantially higher storage requirements.
* Optimization opportunities may include:

  * Reducing dependency footprint.
  * Removing unused functionality.
  * Exploring alternative implementation strategies.
  * Investigating shared libraries or code reuse mechanisms.

#### Understanding Gained

One of the most significant practical challenges of post-quantum cryptography on-chain is not only verification performance but also binary size. While the lock script successfully provides quantum-resistant signature verification, deployment costs become an important consideration when evaluating real-world adoption.

---

## Practical Development Work

### ML-DSA-65 Lock Script Implementation

Implemented and deployed a working post-quantum lock script on CKB testnet.

#### Activities Completed

* Integrated the `fips204` implementation of ML-DSA-65 into a CKB lock script.
* Implemented Blake2b hashing of public keys.
* Built custom witness parsing logic for public key and signature extraction.
* Implemented public key ownership verification.
* Implemented ML-DSA signature verification against transaction hashes.
* Compiled the script for CKB-VM.
* Deployed the lock script to Nervos Testnet.
* Successfully executed and verified transactions using the custom lock.

#### Repository

GitHub Repository:

[ckb-pq-dillithium Repository](https://github.com/beebozy/ckb-pq-dillithium)

#### Testnet Deployment

Transaction Deployment:

[Nervos Testnet Transaction](https://testnet.explorer.nervos.org/transaction/0x4572a31a4b6a3d86396c7f344c5d7d8a51b288c8962bad52179a1724e177ef6b)

---

## Challenges Encountered

During implementation, several challenges emerged:

* Understanding the structure and serialization requirements of ML-DSA public keys and signatures.
* Parsing custom witness formats safely inside a `no_std` environment.
* Managing larger binary sizes introduced by post-quantum cryptographic dependencies.
* Ensuring compatibility between Rust cryptographic libraries and the CKB-VM execution environment.
* Debugging verification failures caused by malformed witness data and serialization mismatches.

These challenges deepened understanding of:

* Cryptographic serialization and deserialization.
* CKB lock script execution flow.
* Witness handling and transaction verification.
* Trade-offs between security guarantees and on-chain resource consumption.

---

## Key Takeaways

By the end of Week 5, I gained:

* Practical experience implementing a post-quantum signature scheme on CKB.
* A stronger understanding of ML-DSA-65 (Dilithium) and lattice-based cryptography.
* Hands-on experience integrating external cryptographic libraries into CKB lock scripts.
* Improved knowledge of witness parsing and transaction verification patterns.
* Experience deploying a custom lock script to Nervos Testnet.
* A deeper appreciation for the storage and deployment trade-offs associated with post-quantum cryptography on blockchain systems.
* Insight into how CKB's flexible scripting model can support future cryptographic standards without requiring protocol changes.
