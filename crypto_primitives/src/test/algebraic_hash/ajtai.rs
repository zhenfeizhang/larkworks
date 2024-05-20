use std::time::Instant;

use ark_std::test_rng;
use ff::Field;
use lark_algebra::{Goldilocks, Polynomial, RingGoldilock256};
use poseidon::Poseidon;

use crate::{AjtaiHash, AlgebraicHash};

#[test]
fn bench_goat_hash() {
    let mut rng = test_rng();
    let repeat = 100;
    let repeat_per_hash = 100;
    let mut hashers = vec![];
    let mut messages_vec = vec![];
    let mut res = vec![];

    for _ in 0..repeat {
        hashers.push(AjtaiHash::setup(&(), &mut rng));
        let mut messages: Vec<[lark_algebra::ZZpX<lark_algebra::ConfigZZpXGoldilocks256>; 4]> =
            vec![];
        for _ in 0..repeat_per_hash {
            messages.push([
                RingGoldilock256::random(&mut rng, None),
                RingGoldilock256::random(&mut rng, None),
                RingGoldilock256::random(&mut rng, None),
                RingGoldilock256::random(&mut rng, None),
            ])
        }
        messages_vec.push(messages)
    }
    let start = Instant::now();
    for (hasher, messages) in hashers.iter().zip(messages_vec.iter()) {
        for msg in messages.iter() {
            res.push(AjtaiHash::hash(msg, hasher));
        }
    }
    println!(
        "goat hash cost {:?} ",
        start.elapsed() / repeat / repeat_per_hash
    );
}

#[test]
fn bench_poseidon_hash() {
    let hasher = Poseidon::<goldilocks::Goldilocks, 12, 11>::new(8, 22);

    let mut rng = test_rng();
    let mut messages = vec![];
    let mut res = vec![];

    let repeat_per_hash = 1000;
    for _ in 0..repeat_per_hash {
        messages.push([
            goldilocks::Goldilocks::random(&mut rng),
            goldilocks::Goldilocks::random(&mut rng),
        ])
    }

    let start = Instant::now();
    for messages in messages.iter() {
        let mut tmp = hasher.clone();
        tmp.update(messages);
        res.push(tmp.squeeze());
    }
    println!(
        "poseidon hash cost {:?} ",
        start.elapsed() / repeat_per_hash
    );
}
