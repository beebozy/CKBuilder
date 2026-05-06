#![no_std]
#![no_main]

use ckb_std::default_alloc;
use ckb_std::entry;

// Pull in our script logic from lib.rs
use ckb_hash_lock::run;

// Set up the heap allocator (required for no_std)
default_alloc!();

// Register the entry point for CKB-VM
entry!(program_entry);

fn program_entry() -> i8 {
    match run() {
        Ok(_) => 0,           // success → return 0
        Err(e) => e as i8,    // failure → return error code
    }
}