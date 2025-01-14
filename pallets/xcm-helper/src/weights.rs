// This file is part of Polkadex.

// Copyright (C) 2020-2023 Polkadex oü.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

//! Autogenerated weights for `xcm_helper`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-28, STEPS: `100`, REPEAT: `200`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `Ubuntu-2204-jammy-amd64-base`, CPU: `Intel(R) Core(TM) i7-7700 CPU @ 3.60GHz`
//! EXECUTION: None, WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 1024

// Executed Command:
// ./parachain-polkadex-node
// benchmark
// pallet
// --pallet
// xcm_helper
// --steps
// 100
// --repeat
// 200
// --extrinsic
// *
// --output
// xcm_helper_weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `xcm_helper`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> crate::XcmHelperWeightInfo for WeightInfo<T> {
    /// Storage: ParachainInfo ParachainId (r:1 w:0)
    /// Proof: ParachainInfo ParachainId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
    /// Storage: XcmHelper ParachainAssets (r:1 w:1)
    /// Proof Skipped: XcmHelper ParachainAssets (max_values: None, max_size: None, mode: Measured)
    /// Storage: XcmHelper WhitelistedTokens (r:1 w:1)
    /// Proof Skipped: XcmHelper WhitelistedTokens (max_values: Some(1), max_size: None, mode: Measured)
    /// Storage: Assets Asset (r:1 w:1)
    /// Proof: Assets Asset (max_values: None, max_size: Some(222), added: 2697, mode: MaxEncodedLen)
    /// The range of component `b` is `[1, 1000]`.
    /// The range of component `b` is `[1, 1000]`.
    fn whitelist_token(_b: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `147`
        //  Estimated: `3687`
        // Minimum execution time: 17_028_000 picoseconds.
        Weight::from_parts(17_753_020, 0)
            .saturating_add(Weight::from_parts(0, 3687))
            .saturating_add(T::DbWeight::get().reads(4))
            .saturating_add(T::DbWeight::get().writes(3))
    }
    /// Storage: ParachainInfo ParachainId (r:1 w:0)
    /// Proof: ParachainInfo ParachainId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
    /// Storage: XcmHelper ParachainAssets (r:1 w:0)
    /// Proof Skipped: XcmHelper ParachainAssets (max_values: None, max_size: None, mode: Measured)
    /// Storage: XcmHelper WhitelistedTokens (r:1 w:1)
    /// Proof Skipped: XcmHelper WhitelistedTokens (max_values: Some(1), max_size: None, mode: Measured)
    /// The range of component `b` is `[1, 1000]`.
    /// The range of component `b` is `[1, 1000]`.
    fn remove_whitelisted_token(b: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `225`
        //  Estimated: `3690`
        // Minimum execution time: 13_237_000 picoseconds.
        Weight::from_parts(13_862_195, 0)
            .saturating_add(Weight::from_parts(0, 3690))
            // Standard Error: 1
            .saturating_add(Weight::from_parts(12, 0).saturating_mul(b.into()))
            .saturating_add(T::DbWeight::get().reads(3))
            .saturating_add(T::DbWeight::get().writes(1))
    }
    /// Storage: System Account (r:2 w:2)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
    /// The range of component `b` is `[1, 1000]`.
    /// The range of component `b` is `[1, 1000]`.
    fn transfer_fee(b: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `226`
        //  Estimated: `6196`
        // Minimum execution time: 25_031_000 picoseconds.
        Weight::from_parts(26_785_061, 0)
            .saturating_add(Weight::from_parts(0, 6196))
            // Standard Error: 18
            .saturating_add(Weight::from_parts(364, 0).saturating_mul(b.into()))
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().writes(2))
    }
}
