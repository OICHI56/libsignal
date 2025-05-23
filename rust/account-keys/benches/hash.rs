//
// Copyright 2023 Signal Messenger, LLC.
// SPDX-License-Identifier: AGPL-3.0-only
//

use const_str::hex;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn hash(c: &mut Criterion) {
    // let mut group = c.benchmark_group("pin_hash");

    let password = b"password";
    let salt = hex!("000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f");
    let verification_string = libsignal_account_keys::local_pin_hash(password).unwrap();

    c.bench_function("svr_hash", |b| {
        b.iter(|| libsignal_account_keys::PinHash::create(password, &salt).unwrap());
    });

    c.bench_function("verification_hash", |b| {
        b.iter(|| libsignal_account_keys::local_pin_hash(password).unwrap());
    });

    c.bench_function("verify", |b| {
        b.iter(|| {
            libsignal_account_keys::verify_local_pin_hash(&verification_string, password).unwrap()
        });
    });
}

criterion_group!(benches, hash);
criterion_main!(benches);
