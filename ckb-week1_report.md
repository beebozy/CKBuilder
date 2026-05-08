# CKB Learning Report — Week 1

## Introduction

This week focused on understanding the fundamentals of the Nervos CKB blockchain architecture and beginning practical smart contract development using Rust. The goal was to build a strong conceptual foundation before moving deeper into advanced script development and protocol-level mechanics.

---

# Topics Covered

## 1. Nervos Blockchain Fundamentals

Studied the overall architecture and philosophy of the Nervos Network and the Common Knowledge Base (CKB).

### Key Learnings

* Nervos separates state storage from computation.
* CKB is designed as a decentralized store of cryptographic state.
* The blockchain uses the Cell Model instead of the account model.
* Layer 1 prioritizes security and decentralization while Layer 2 handles scalability.

### Resource

* [Nervos Blockchain Fundamentals](https://docs.nervos.org/docs/ckb-fundamentals/nervos-blockchain?utm_source=chatgpt.com)

---

## 2. CKB vs Bitcoin

Explored the architectural similarities and differences between CKB and Bitcoin.

### Key Learnings

### Similarities

* Both use a UTXO-inspired transaction structure.
* Both prioritize decentralization and security.
* Both use Proof of Work consensus.

### Differences

* CKB extends Bitcoin’s UTXO model into the Cell Model.
* Cells can store arbitrary state and executable scripts.
* CKB supports programmable verification logic through scripts.
* CKB is designed as a generalized cryptographic state machine.

### Resource

* [CKB vs Bitcoin](https://docs.nervos.org/docs/ckb-fundamentals/ckb-vs-btc?utm_source=chatgpt.com)

---

## 3. Cell Model

Studied the Cell Model, which is one of the most important concepts in CKB.

### Key Learnings

* Cells are immutable objects containing:

  * Capacity
  * Lock script
  * Type script
  * Data
* Cells are consumed and recreated through transactions.
* Lock scripts control ownership.
* Type scripts validate state transitions.

### Understanding Gained

The Cell Model combines ideas from:

* Bitcoin’s UTXO model
* Smart contract programmability
* Stateful verification systems

This model enables highly flexible and composable blockchain applications.

---

## 4. CKB-VM

Learned about the CKB Virtual Machine and how scripts execute inside the network.

### Key Learnings

* CKB-VM is based on RISC-V architecture.
* Scripts are deterministic and sandboxed.
* Contracts are compiled into RISC-V binaries.
* CKB-VM minimizes complexity while maintaining flexibility.

### Resource

* [CKB-VM Fundamentals](https://docs.nervos.org/docs/ckb-fundamentals/ckb-vm?utm_source=chatgpt.com)

---

## 5. Consensus Mechanism

Studied the consensus model used by Nervos CKB.

### Key Learnings

* CKB uses Proof of Work (PoW).
* The consensus protocol is called NC-Max.
* It is optimized for security and decentralized block propagation.
* Uncle blocks are incorporated to improve chain efficiency.

### Resource

* [CKB Consensus](https://docs.nervos.org/docs/ckb-fundamentals/consensus?utm_source=chatgpt.com)

---

## 6. CKB Addresses

Learned how CKB addresses are constructed and encoded.

### Key Learnings

* Addresses encode script information.
* Different address formats exist for:

  * short payloads
  * full payloads
* Address generation is tied directly to script hashes.

### Resource

* [CKB Address Format](https://docs.nervos.org/docs/ckb-fundamentals/ckb-address?utm_source=chatgpt.com)

---

## 7. Hashing in CKB

Studied hashing mechanisms used throughout the protocol.

### Key Learnings

* Hashes are used for:

  * transaction identification
  * block identification
  * script verification
  * Merkle structures
* CKB uses cryptographic hashing heavily for integrity and verification.
* Learned about preimage resistance and hash-based verification concepts.

### Resource

* [CKB Hashes](https://docs.nervos.org/docs/ckb-fundamentals/ckbhash?utm_source=chatgpt.com)

---

# Practical Development Work

## Rust Script Development

Began practical contract development using Rust and the CKB script templates.

### Activities Completed

* Set up a Rust-based CKB contract workspace
* Learned the CKB contract build pipeline
* Worked with RISC-V compilation targets
* Explored script template structure
* Understood simulator-based testing workflow

---

## Multisig Script

Implemented a basic multisignature script as part of the scripting course.

### Concepts Practiced

* Script structure
* Verification logic
* Witness handling
* Lock script development
* Rust contract organization

### Repository

* [Multisig Script Repository](https://github.com/beebozy/CKBuilder/tree/main/ckb-rust-script/contracts/multisig?utm_source=chatgpt.com)

---

# Challenges Encountered

During development, several environment and dependency issues were encountered:

* LLVM compilation issues
* RISC-V target configuration
* Molecule and bytes crate compatibility
* Atomic instruction incompatibilities in CKB-VM
* Workspace configuration problems

These challenges helped deepen understanding of:

* CKB’s low-level architecture
* Rust no_std development
* RISC-V limitations
* CKB build tooling

---

# Key Takeaways

By the end of Week 1, I gained:

* A foundational understanding of the Nervos ecosystem
* A deeper appreciation for the Cell Model
* Exposure to CKB-VM and RISC-V based execution
* Initial experience writing and organizing CKB scripts in Rust
* Better understanding of blockchain verification models and hash-based systems

