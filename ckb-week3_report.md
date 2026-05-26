# CKB Learning Report — Week 3

## Introduction

This week focused on intermediate CKB script programming, moving beyond foundational concepts into practical scripting patterns used in real-world CKB applications. The goal was to understand how scripts interact with transaction data through syscalls, how to build User Defined Tokens (UDT), and how to leverage WebAssembly within the CKB execution environment. These topics represent a significant step forward in writing production-level CKB scripts.

---

## Topics Covered

### 1. CKB Script Basics (Intermediate)

Studied the deeper mechanics of how scripts are structured, deployed, and executed on CKB.

#### Key Learnings

- Distinguished between **script** (the on-chain data structure) and **script code** (the compiled RISC-V binary)
- The `Script` struct contains three fields: `code_hash`, `hash_type`, and `args`
- CKB executes scripts as standard Unix-style RISC-V executables — a return code of `0` means success; any non-zero return code is a failure
- Script code is deployed to CKB by storing it in a cell's data field, then referenced via `code_hash` in other scripts
- `hash_type` determines how `code_hash` is resolved — either by data hash or by type script hash
- Scripts can be written in any language that compiles to RISC-V, including C and Rust
- Explored the always-success and always-failure script patterns as foundational building blocks

#### Understanding Gained

Script code is treated like a normal program running on a minimal computer. This mental model — RISC-V binary + return code = pass/fail — makes CKB's verification model both simple and powerful. Any language targeting RISC-V can be used to write CKB scripts, opening the door to a wide range of tooling and languages.

#### Resource

[Introduction to CKB Script Programming 2: Script Basics](https://xuejie.space/2019_07_13_introduction_to_ckb_script_programming_script_basics/)

---

### 2. User Defined Tokens (UDT)

Explored how to implement custom fungible tokens on CKB using type scripts, analogous to ERC-20 tokens on Ethereum.

#### Key Learnings

- CKB tokens are called **User Defined Tokens (UDT)** — CKB's answer to ERC-20
- Unlike Ethereum, UDT balances are **distributed across many cells**, not stored in a single contract account
- A UDT cell structure follows this pattern:
  - `data`: stores the token amount as a `uint128` (little-endian)
  - `type`: the UDT type script, with `args` containing the owner's lock script hash
  - `lock`: user-defined lock script
- A minimal UDT type script enforces two rules:
  - In a transfer, the sum of UDT amounts in output cells must equal the sum in input cells
  - Only the token issuer (identified by lock script hash in `args`) can mint new tokens during initial creation
- Each unique type script represents a unique token — two cells sharing the same type script are the same UDT
- The distributed cell design avoids the single-cell bottleneck problem that a centralised balance store would create

#### Understanding Gained

The Cell Model enables a fundamentally different token design compared to Ethereum. Rather than a single shared state, each holder's balance exists in its own cell. This makes UDT transfers more composable and eliminates contention on a single storage slot, though it requires careful type script logic to ensure token conservation across transactions.

#### Resource

[Introduction to CKB Script Programming 3: UDT](https://xuejie.space/2019_09_06_introduction_to_ckb_script_programming_udt/)

---

### 3. WebAssembly (WASM) on CKB

Studied how WebAssembly can be compiled and executed within the CKB-VM environment via RISC-V.

#### Key Learnings

- CKB-VM is RISC-V based, but WASM modules can run on CKB by **compiling a WASM interpreter to RISC-V**
- This approach allows developers to write scripts in Rust (targeting WASM), then run them inside CKB via an embedded interpreter
- The WASM-on-CKB path demonstrates CKB's language agnosticism — the VM doesn't need to understand WASM natively
- Explored the Fibonacci example as a practical demonstration of WASM execution on CKB-VM
- A standalone CKB-VM binary in the test suite can be used to verify WASM programs locally before on-chain deployment
- Performance trade-offs exist: WASM-via-interpreter is slower than direct RISC-V binaries, but an LLVM-based compilation path offers a higher-performance alternative for the future
- Type scripts in particular are good candidates for Rust/WASM development, since they are often less performance-critical than lock scripts

#### Understanding Gained

Running WASM inside CKB-VM is a powerful demonstration of CKB's design philosophy: keep the base layer minimal and let developers bring their own toolchains. This opens the door to writing CKB scripts in virtually any language with a WASM compilation target, broadening the developer ecosystem significantly.

#### Resource

[Introduction to CKB Script Programming 4: WebAssembly on CKB](https://xuejie.space/2019_10_09_introduction_to_ckb_script_programming_wasm_on_ckb/)

---

### 4. CKB VM Syscalls

Studied how scripts communicate with the CKB node process to read transaction data during execution.

#### Key Learnings

- **Syscalls** are the mechanism by which scripts running inside CKB-VM access external data — they follow the standard RISC-V syscall convention
- All syscalls (except `Exit`) use a **partial loading** design, allowing scripts to read data in chunks using three core parameters: `addr` (buffer pointer), `len` (buffer length), and `offset` (read offset)
- Key syscalls studied:

  | Syscall | Purpose |
  |---|---|
  | `ckb_load_tx_hash` | Load the hash of the current transaction |
  | `ckb_load_transaction` | Load the full serialized transaction (Molecule format) |
  | `ckb_load_script_hash` | Load the hash of the currently running script |
  | `ckb_load_script` | Load the currently running script structure |
  | `ckb_load_cell` | Load a specific cell by index and source |
  | `ckb_load_input` | Load a specific transaction input |
  | `ckb_load_cell_data` | Load the data field of a cell |
  | `ckb_load_witness` | Load a witness from the transaction |

- **Source values** control where data is read from: `CKB_SOURCE_INPUT`, `CKB_SOURCE_OUTPUT`, `CKB_SOURCE_CELL_DEP`, and `CKB_SOURCE_GROUP_INPUT` (a virtual array scoped to the current script group)
- `CKB_SOURCE_GROUP_INPUT` is particularly important for lock scripts — it limits reads to inputs that share the current lock script, enabling group-scoped verification
- Cycles are consumed per instruction and per syscall, with a hard block-level cycle limit enforced at consensus

#### Understanding Gained

Syscalls are the bridge between deterministic script execution and live transaction context. Understanding the partial loading pattern and source types is essential for writing any non-trivial script that needs to inspect its own arguments, validate cell balances, or check witness data. The group-scoped source is especially powerful for lock scripts that must verify signatures across multiple inputs in a single pass.

#### Resource

[CKB VM Syscalls — Nervos Docs](https://docs.nervos.org/docs/script/syscalls-for-script)

---

## Practical Development Work

### Script Interaction Patterns

Applied syscall knowledge to understand how intermediate scripts are structured in practice.

#### Activities Completed

- Traced how a UDT type script uses `ckb_load_cell_data` to iterate over input and output cells and sum token amounts
- Studied how lock scripts use `CKB_SOURCE_GROUP_INPUT` to scope witness and input reads to the current script group
- Analysed how `ckb_load_script` is used to retrieve the running script's own `args` field for owner verification in UDT scripts
- Explored how cycles are budgeted across complex multi-input transactions

---

## Challenges Encountered

During this week's study, several conceptual challenges arose:

- **Partial loading pattern**: Understanding the three-argument (`addr`, `len`, `offset`) interface required careful study before the pattern felt natural
- **Source type scoping**: Distinguishing between `CKB_SOURCE_INPUT` (all inputs) and `CKB_SOURCE_GROUP_INPUT` (script-group-scoped inputs) was  confusing 
- **UDT cell iteration**: Implementing correct balance-sum logic across variable numbers of input and output cells requires careful index-based looping with syscalls
- **WASM interpreter overhead**: Understanding why WASM-on-CKB has performance trade-offs compared to native RISC-V binaries

These challenges deepened understanding of:

- CKB-VM's memory and data access model
- The importance of script group scoping in multi-input transactions
- How the Cell Model shapes token design at the protocol level
- The execution cost model and its impact on script design

---

## Key Takeaways

By the end of Week 3, I gained:

- A solid some understanding of intermediate CKB script structure and the script vs. script code distinction
- Practical knowledge of how UDTs are designed and enforced using type scripts
- Exposure to running WebAssembly inside CKB-VM as a demonstration of CKB's language flexibility
- Working knowledge of the CKB syscall interface and how scripts read transaction data
- A clearer mental model of how source types and partial loading work together to enable complex verification logic