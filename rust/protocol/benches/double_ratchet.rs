// Copyright 2025 Signal Messenger, LLC
// SPDX-License-Identifier: AGPL-3.0-only

use libsignal_core::ProtocolAddress;
use libsignal_protocol::*;
use rand::TryRngCore;
use rand_core::OsRng;


use criterion::{criterion_group, criterion_main, Criterion};
use futures_util::FutureExt;

#[path = "../tests/support/mod.rs"]
mod support;

//fn state(dir: Direction) -> SerializedState {
//    initial_state(Params {
//        version: Version::MAX,
//        min_version: Version::MAX,
//        auth_key: b"1",
//        direction: dir,
//        chain_params: ChainParams::default(),
//    })
//    .expect("should be valid params")
//}

//#[bench]
//fn init_a(b: &mut Bencher) {
//    b.iter(|| {
//        // Inner closure, the actual test
//        state(Direction::A2B);
//    });
//}
//#[bench]
//fn init_b(b: &mut Bencher) {
//    b.iter(|| {
//        // Inner closure, the actual test
//        black_box(state(Direction::B2A));
//    });
//}

struct CKAState {
    store: InMemSignalProtocolStore,
    address: ProtocolAddress
}

fn init_states() -> (CKAState, CKAState) {
    let (alice_session_record, bob_session_record) = support::initialize_sessions_v3().unwrap();

    let alice_address = ProtocolAddress::new("+14159999999".to_owned(), 1.into());
    let bob_address = ProtocolAddress::new("+14158888888".to_owned(), 1.into());

    let mut alice_store = support::test_in_memory_protocol_store().unwrap();
    let mut bob_store = support::test_in_memory_protocol_store().unwrap();

    alice_store
        .store_session(&bob_address, &alice_session_record)
        .now_or_never()
        .expect("sync").unwrap();
    bob_store
        .store_session(&alice_address, &bob_session_record)
        .now_or_never()
        .expect("sync").unwrap();

    let alice_state = CKAState {store: alice_store, address: alice_address};
    let bob_state = CKAState {store: bob_store, address: bob_address};
    (alice_state, bob_state)
}

pub fn send_recv_result(c: &mut Criterion) -> Result<(), SignalProtocolError>{
    let mut ctr: u64 = 0;

    //let mut a = state(Direction::A2B);
    //let mut b = state(Direction::B2A);
    let (mut alice_state, mut bob_state) = init_states();

    let mut rng = OsRng.unwrap_err();
    let mut old_key_a = [42; 32];
    c.bench_function("double ratchet send + recv", |b| {
        b.iter(|| {
            ctr += 1;
            let (x_state, y_state) = if ctr % 2 == 1 {
                (&mut alice_state, &mut bob_state)
            } else {
                (&mut bob_state, &mut alice_state)
            };
            let (msg, key_a) = support::ckasend(&mut x_state.store, &y_state.address, ctr);
            let key_b = support::ckarecv(&mut y_state.store, &msg, &x_state.address, &mut rng);
            assert_eq!(key_a, key_b);
            assert_ne!(key_b, old_key_a);
            old_key_a = key_a;
        })
    });
    Ok(())
}

//#[bench]
/*fn long_chain_send(c: &mut Criterion) -> Result<(), SignalProtocolError> {
    let mut rng = OsRng.unwrap_err();
    let (mut a, mut b) = init_states();


    // Build a state with a lot of unused chain keys.
    for _i in 0..8 {
        for _j in 0..24000 {
            let _ = support::ckasend(&mut a.store, &b.address, 0);
            //let Send { state, .. } = send(&a, &mut rng).unwrap();
            //a = state;
        }
        let (msg, key_a) = support::ckasend(&mut a.store, &b.address, 0);
        //a = state;
        let key_b = support::ckarecv(&mut b.store, &msg, &a.address, &mut rng);
        //let Recv { state, .. } = recv(&b, &msg).unwrap();
        //b = state;
        assert_eq!(key_a, key_b)
    }

    //println!("state size: {}", );
    let (msg, _ ) = support::ckasend(&mut a.store, &b.address, 0);
    c.bench_function("double ratchet long chain send", |bench| {
        bench.iter(|| {
            let _ = support::ckarecv(&mut b.store, &msg, &a.address, &mut rng);
        });
    });
    Ok(())
}*/

pub fn send_recv(c: &mut Criterion) {
    send_recv_result(c).expect("success");
}

criterion_group!(benches, send_recv);

criterion_main!(benches);
