#![cfg_attr(not(any(feature = "library", test)), no_std)]
#![cfg_attr(not(test), no_main)]

#[cfg(any(feature = "library", test))]
extern crate alloc;

#[cfg(not(any(feature = "library", test)))]
ckb_std::entry!(program_entry);
#[cfg(not(any(feature = "library", test)))]
ckb_std::default_alloc!(16384, 1258306, 64);

use ckb_std::{
    ckb_constants::Source,
    high_level::{load_script, load_tx_hash, load_witness_args},
};
use blake2b_ref::{Blake2b, Blake2bBuilder};
use secp256k1::{ecdsa::RecoverableSignature, ecdsa::RecoveryId, Message, Secp256k1};


// Each signature is 65 bytes (64 bytes + 1 recovery id byte)
const SIG_LEN: usize = 65;
// Each pubkey hash is 20 bytes (first 20 bytes of blake2b hash of pubkey)
const PUBKEY_HASH_LEN: usize = 20;
// CKB's blake2b personal string
const CKB_HASH_PERSONALIZATION: &[u8] = b"ckb-default-hash";
// Maximum number of pubkeys supported
const MAX_KEYS: usize = 16;


// Error
#[repr(i8)]
pub enum Error {
    InvalidArgs      = 1,
    InvalidWitness   = 2,
    InvalidSignature = 3,
    BelowThreshold   = 4,
}

pub fn program_entry() -> i8 {
    match run() {
        Ok(_)  => 0,
        Err(e) => e as i8,
    }
}

fn run() -> Result<(), Error> {
  
    // args layout:
    //   byte 0     = threshold M (how many sigs needed)
    //   bytes 1..  = N pubkey hashes, each 20 bytes
    //
    // Example 2-of-3:
    //   args = [0x02, <hash1(20)>, <hash2(20)>, <hash3(20)>]

    let script = load_script().map_err(|_| Error::InvalidArgs)?;
    let args = script.args();
    let args_bytes = args.raw_data();

    // need at least threshold byte + one pubkey hash
    if args_bytes.len() < 1 + PUBKEY_HASH_LEN {
        return Err(Error::InvalidArgs);
    }

    let threshold = args_bytes[0] as usize;

    // remaining bytes must divide evenly into pubkey hashes
    let pubkey_data = &args_bytes[1..];
    if pubkey_data.len() % PUBKEY_HASH_LEN != 0 {
        return Err(Error::InvalidArgs);
    }

    let n = pubkey_data.len() / PUBKEY_HASH_LEN;

    // threshold must be sane: at least 1 and no more than n
    if threshold == 0 || threshold > n || n > MAX_KEYS {
        return Err(Error::InvalidArgs);
    }

    // collect pubkey hashes
    let mut pubkey_hashes = [[0u8; PUBKEY_HASH_LEN]; MAX_KEYS];
    for i in 0..n {
        pubkey_hashes[i].copy_from_slice(
            &pubkey_data[i * PUBKEY_HASH_LEN..(i + 1) * PUBKEY_HASH_LEN],
        );
    }

    // load witness 
    // witness.lock layout:
    //   [sig1 (65 bytes)][sig2 (65 bytes)]...

    let witness = load_witness_args(0, Source::GroupInput)
        .map_err(|_| Error::InvalidWitness)?;

    let lock_field = witness
        .lock()
        .to_opt()
        .ok_or(Error::InvalidWitness)?;

    let sigs_bytes = lock_field.raw_data();

    if sigs_bytes.is_empty() || sigs_bytes.len() % SIG_LEN != 0 {
        return Err(Error::InvalidWitness);
    }

    let sig_count = sigs_bytes.len() / SIG_LEN;

  // load tx hash 
    let tx_hash = load_tx_hash().map_err(|_| Error::InvalidWitness)?;

    // verify signature 
    let mut valid_count = 0usize;
    let secp = Secp256k1::verification_only();

    'outer: for i in 0..sig_count {
        let sig = &sigs_bytes[i * SIG_LEN..(i + 1) * SIG_LEN];

        // recover pubkey hash from this signature
        let recovered_hash = match recover_pubkey_hash(&secp, sig, &tx_hash) {
            Ok(h)  => h,
            Err(_) => continue, // bad sig, skip
        };

        // check if it matches any registered pubkey hash
        for j in 0..n {
            if pubkey_hashes[j] == recovered_hash {
                valid_count += 1;
                continue 'outer; // don't double count same key
            }
        }
    }

  // check threshold 
    if valid_count < threshold {
        return Err(Error::BelowThreshold);
    }

    Ok(())
}

// helper functions 

// Build a CKB blake2b hasher
fn new_blake2b() -> Blake2b {
    Blake2bBuilder::new(32)
        .personal(CKB_HASH_PERSONALIZATION)
        .build()
}

// Hash data with CKB blake2b, return 32-byte digest
fn blake2b_256(data: &[u8]) -> [u8; 32] {
    let mut hasher = new_blake2b();
    hasher.update(data);
    let mut out = [0u8; 32];
    hasher.finalize(&mut out);
    out
}

fn recover_pubkey_hash(
    secp: &Secp256k1<secp256k1::VerifyOnly>,
    sig: &[u8],
    msg_hash: &[u8; 32],
) -> Result<[u8; PUBKEY_HASH_LEN], Error> {
    // last byte is recovery id — use try_from instead of from_i32
    let rec_id = RecoveryId::try_from(sig[64] as i32)
        .map_err(|_| Error::InvalidSignature)?;

    let rec_sig = RecoverableSignature::from_compact(&sig[..64], rec_id)
        .map_err(|_| Error::InvalidSignature)?;

    let message = Message::from_digest(*msg_hash);

    let pubkey = secp
        .recover_ecdsa(&message, &rec_sig)
        .map_err(|_| Error::InvalidSignature)?;

    let pubkey_bytes = pubkey.serialize();

    let hash = blake2b_256(&pubkey_bytes);
    let mut out = [0u8; PUBKEY_HASH_LEN];
    out.copy_from_slice(&hash[..PUBKEY_HASH_LEN]);

    Ok(out)
}