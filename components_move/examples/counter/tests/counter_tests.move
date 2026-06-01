// Copyright (c) 2026 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[test_only]
module tf_components::example_counter_tests;

use iota::test_scenario as ts;
use std::string;
use std::option::none;
use tf_components::{
    capability::Capability,
    counter::{Self, Counter},
    counter_permission as permission
};

/// Creates a Counter with a "counter-admin" role restricted to Wednesday,
/// issues a capability for that role to `counter_admin_user`, and returns
/// the scenario along with the issued capability's ID.
fun prepare_counter_and_issue_capability(
    super_admin_user: address,
    counter_admin_user: address,
): (ts::Scenario, ID) {
    let mut scenario = ts::begin(super_admin_user);

    // Setup: Create Counter
    {
        let (super_admin_cap, _counter_id) = counter::create(ts::ctx(&mut scenario));
        transfer::public_transfer(super_admin_cap, super_admin_user);
    };

    // Create an additional CounterAdmin role only valid on Wednesday
    ts::next_tx(&mut scenario, super_admin_user);
    {
        let super_admin_cap = ts::take_from_sender<Capability>(&scenario);
        let mut counter = ts::take_shared<Counter>(&scenario);
        let clock = iota::clock::create_for_testing(ts::ctx(&mut scenario));

        counter
            .access_mut()
            .create_role(
                &super_admin_cap,
                string::utf8(b"counter-admin"),
                permission::counter_admin_permissions(),
                std::option::some(counter::wednesday()),
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

        iota::clock::destroy_for_testing(clock);
        ts::return_to_sender(&scenario, super_admin_cap);
        ts::return_shared(counter);

        counter_admin_cap_id
    };

    (scenario, counter_admin_cap_id)
}

/// Test capability lifecycle: creation, usage, revocation and destruction in a complete workflow.
#[test]
fun test_capability_lifecycle() {
    let super_admin_user = @0xAD;
    let counter_admin_user = @0xB0B;

    let (mut scenario, counter_admin_cap_id) = prepare_counter_and_issue_capability(
        super_admin_user,
        counter_admin_user,
    );

    // Use CounterAdmin capability on Wednesday to increment the counter
    ts::next_tx(&mut scenario, counter_admin_user);
    {
        let counter_admin_cap = ts::take_from_sender<Capability>(&scenario);
        let mut counter = ts::take_shared<Counter>(&scenario);
        let mut clock = iota::clock::create_for_testing(ts::ctx(&mut scenario));
        let ms_per_day: u64 = 86_400_000;
        clock.set_for_testing(ms_per_day * 6 + 1); // Set to the first ms on the first Wednesday after Unix epoch which happened on a Thursday

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

        // Make sure there are no revoked capabilities so far
        assert!(counter.access().revoked_capabilities().length() == 0, 0);

        counter
            .access_mut()
            .revoke_capability(
                &super_admin_cap,
                counter_admin_cap_id,
                none(),
                &clock,
                ts::ctx(&mut scenario),
            );

        // Verify capability has been added to the revoked_capabilities list
        assert!(counter.access().revoked_capabilities().length() == 1, 1); // counter-admin only
        assert!(counter.access().revoked_capabilities().contains(counter_admin_cap_id), 2);

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

/// Test that a capability associated with a role restricted to Wednesday cannot be used on Monday.
#[test]
#[expected_failure(abort_code = counter::EWeekDayMismatch)]
fun test_wednesday_role_rejected_on_monday() {
    let super_admin_user = @0xAD;
    let counter_admin_user = @0xB0B;
    let ms_per_day: u64 = 86_400_000;

    let (mut scenario, _counter_admin_cap_id) = prepare_counter_and_issue_capability(
        super_admin_user,
        counter_admin_user,
    );

    // Attempt to use the capability on Monday — should fail with EWeekDayMismatch
    ts::next_tx(&mut scenario, counter_admin_user);
    {
        let counter_admin_cap = ts::take_from_sender<Capability>(&scenario);
        let mut counter = ts::take_shared<Counter>(&scenario);
        let mut clock = iota::clock::create_for_testing(ts::ctx(&mut scenario));
        // Day 4 from epoch = Monday (epoch day 0 = Thursday(3), +4 days = Monday(0))
        clock.set_for_testing(ms_per_day * 4 + 1);

        counter.increment(
            &counter_admin_cap,
            &clock,
            ts::ctx(&mut scenario),
        );

        iota::clock::destroy_for_testing(clock);
        ts::return_to_sender(&scenario, counter_admin_cap);
        ts::return_shared(counter);
    };

    ts::end(scenario);
}
