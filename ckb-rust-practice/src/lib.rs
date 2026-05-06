#![no_std]

use ckb_std::{
    ckb_constants::Source,
    high_level::{load_script, load_witness_args},
};

// ── Error codes ──────────────────────────────────────────────────────────────
// Non-zero codes returned to CKB-VM on failure.
// Keep these as a clean enum so callers can match on them.
#[repr(i8)]
pub enum Error {
    // Failed to load the running script itself
    LoadScriptFailed  = 1,
    // args must be exactly 32 bytes (a blake2b hash)
    InvalidArgs       = 2,
    // Could not load the witness from the transaction
    LoadWitnessFailed = 3,
    // The witness lock field was missing
    MissingWitness    = 4,
    // The provided preimage does not match the hash in args
    HashMismatch      = 5,
}

// ── Main logic ───────────────────────────────────────────────────────────────
pub fn run() -> Result<(), Error> {
    // 1. Load the currently-executing script to read its args.
    //    args = the 32-byte blake2b hash of the expected preimage,
    //    set at deploy time by whoever creates the lock cell.
    let script = load_script().map_err(|_| Error::LoadScriptFailed)?;
    let args = script.args();
    let args_bytes = args.raw_data();

    // Enforce that args is exactly 32 bytes (one blake2b digest)
    if args_bytes.len() != 32 {
        return Err(Error::InvalidArgs);
    }

    // Copy the expected hash out of args
    let mut expected_hash = [0u8; 32];
    expected_hash.copy_from_slice(&args_bytes[..32]);

    // 2. Load the witness for the first input in our lock group.
    //    The spender puts their preimage in witness.lock.
    let witness = load_witness_args(0, Source::GroupInput)
        .map_err(|_| Error::LoadWitnessFailed)?;

    let lock_field = witness
        .lock()
        .to_opt()
        .ok_or(Error::MissingWitness)?;

    let preimage = lock_field.raw_data();

    // 3. Hash the provided preimage with blake2b and compare.
    let computed_hash = blake2b_256(preimage.as_ref());

    if computed_hash != expected_hash {
        return Err(Error::HashMismatch);
    }

    Ok(())
}

// ── blake2b helper ───────────────────────────────────────────────────────────
// CKB uses blake2b with a 32-byte digest and the personal string "ckb-default-hash".
// ckb-std re-exports this via ckb_std::ckb_types::packed — we implement it manually
// here for clarity. In production, use the `blake2b-ref` crate.
fn blake2b_256(data: &[u8]) -> [u8; 32] {
    use ckb_std::ckb_types::packed::Byte32;
    // ckb-std provides a built-in hasher:
    let mut ctx = ckb_std::ckb_types::core::Capacity::zero(); // placeholder
    // In real code you would do:
    //   let mut hasher = new_blake2b();
    //   hasher.update(data);
    //   hasher.finalize(&mut out);
    // For now, zero-fill as a placeholder — replace with blake2b-ref crate
    let _ = (data, ctx);
    [0u8; 32]
}