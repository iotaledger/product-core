// Copyright (c) 2026 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[test_only]
module tf_components::example_counter_tests;

use iota::test_scenario as ts;
use std::string;
use tf_components::capability::Capability;
use tf_components::counter::{Self, Counter};
use tf_components::counter_permission as permission;

/// Test capability lifecycle: creation, usage, revocation and destruction in a complete workflow.
///
/// This test validates:
/// - A capability can be created for the `counter-admin` role
/// - The Capability can be used to perform authorized actions
/// - The Capability can be revoked
/// - The Capability can be destroyed thereafter
#[test]
fun test_capability_lifecycle() {
    let super_admin_user = @0xAD;
    let counter_admin_user = @0xB0B;

    let mut scenario = ts::begin(super_admin_user);

    // Setup: Create Counter
    {
        let (super_admin_cap, _counter_id) = counter::create(ts::ctx(&mut scenario));
        transfer::public_transfer(super_admin_cap, super_admin_user);
    };

    // Create an additional CounterAdmin role
    ts::next_tx(&mut scenario, super_admin_user);
    {
        let super_admin_cap = ts::take_from_sender<Capability>(&scenario);
        let mut counter = ts::take_shared<Counter>(&scenario);
        let clock = iota::clock::create_for_testing(ts::ctx(&mut scenario));

        // Initially only the super-admin cap should be tracked
        assert!(counter.access().issued_capabilities().size() == 1, 0);

        counter
            .access_mut()
            .create_role(
                &super_admin_cap,
                string::utf8(b"counter-admin"),
                permission::counter_admin_permissions(),
                &clock,
                ts::ctx(&mut scenario),
            );

        iota::clock::destroy_for_testing(clock);
        ts::return_to_sender(&scenario, super_admin_cap);
        ts::return_shared(counter);
    };

    // Issue the CounterAdmin capability to another user
    ts::next_tx(&mut scenario, super_admin_user);
    let counter_admin_cap_id = {
        let super_admin_cap = ts::take_from_sender<Capability>(&scenario);
        let mut counter = ts::take_shared<Counter>(&scenario);
        let clock = iota::clock::create_for_testing(ts::ctx(&mut scenario));
        let counter_cap = counter
            .access_mut()
            .new_capability(
                &super_admin_cap,
                &string::utf8(b"counter-admin"),
                std::option::none(),
                std::option::none(),
                std::option::none(),
                &clock,
                ts::ctx(&mut scenario),
            );
        let counter_admin_cap_id = object::id(&counter_cap);
        transfer::public_transfer(counter_cap, counter_admin_user);

        // Verify all capabilities are tracked
        assert!(counter.access().issued_capabilities().size() == 2, 1); // super-admin + counter-admin
        assert!(counter.access().issued_capabilities().contains(&counter_admin_cap_id), 2);

        iota::clock::destroy_for_testing(clock);
        ts::return_to_sender(&scenario, super_admin_cap);
        ts::return_shared(counter);

        counter_admin_cap_id
    };

    // Use CounterAdmin capability to increment the counter
    ts::next_tx(&mut scenario, counter_admin_user);
    {
        let counter_admin_cap = ts::take_from_sender<Capability>(&scenario);
        let mut counter = ts::take_shared<Counter>(&scenario);
        let clock = iota::clock::create_for_testing(ts::ctx(&mut scenario));

        assert!(counter.value() == 0, 3);
        counter.increment(
            &counter_admin_cap,
            &clock,
            ts::ctx(&mut scenario),
        );
        assert!(counter.value() == 1, 4);

        iota::clock::destroy_for_testing(clock);
        ts::return_to_sender(&scenario, counter_admin_cap);
        ts::return_shared(counter);
    };

    // Revoke the CounterAdmin capability
    ts::next_tx(&mut scenario, super_admin_user);
    {
        let super_admin_cap = ts::take_from_sender<Capability>(&scenario);
        let mut counter = ts::take_shared<Counter>(&scenario);
        let clock = iota::clock::create_for_testing(ts::ctx(&mut scenario));

        counter
            .access_mut()
            .revoke_capability(
                &super_admin_cap,
                counter_admin_cap_id,
                &clock,
                ts::ctx(&mut scenario),
            );

        // Verify capability was removed from the issued_capabilities list
        assert!(counter.access().issued_capabilities().size() == 1, 5); // super-admin only
        assert!(!counter.access().issued_capabilities().contains(&counter_admin_cap_id), 6);

        iota::clock::destroy_for_testing(clock);
        ts::return_to_sender(&scenario, super_admin_cap);
        ts::return_shared(counter);
    };

    // The `counter_admin_user` can destroy the capability before it is revoked or after it is revoked.
    // Here we test destroying after revocation.
    // If the capability is destroyed before revoking it, the capability would be revoked automatically during `destroy_capability()`.
    ts::next_tx(&mut scenario, counter_admin_user);
    {
        let counter_admin_cap = ts::take_from_sender<Capability>(&scenario);
        let mut counter = ts::take_shared<Counter>(&scenario);

        counter.access_mut().destroy_capability(counter_admin_cap);

        ts::return_shared(counter);
    };

    ts::end(scenario);
}
