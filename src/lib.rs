// lib.rs
#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
#![cfg_attr(not(any(test, feature = "export-abi")), no_std)]
extern crate alloc;

// Import untuk ABI export
use alloc::vec;
use alloc::vec::Vec;

// Import dari Stylus SDK
use stylus_sdk::prelude::*;
use stylus_sdk::storage::StorageU256;
use stylus_sdk::alloy_primitives::U256;

// Definisi storage contract
#[storage]
#[entrypoint]
pub struct Counter {
    count: StorageU256,  // Variabel untuk menyimpan counter
}

#[public]
impl Counter {
    // Fungsi untuk increment counter (+1)
    pub fn increment(&mut self) {
        let current = self.count.get();           // Baca nilai sekarang
        let new_value = current + U256::from(1);  // Tambah 1
        self.count.set(new_value);                // Simpan nilai baru
    }

    // Fungsi untuk membaca nilai counter
    pub fn get_count(&self) -> U256 {
        self.count.get()  // Return nilai counter
    }

    // Fungsi untuk reset counter ke 0
    pub fn reset(&mut self) {
        self.count.set(U256::from(0));  // Set ke 0
    }
}