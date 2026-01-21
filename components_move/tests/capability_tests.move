// Copyright (c) 2026 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[test_only]
module tf_components::capability_tests;

use tf_components::test_utils::{Self, initial_admin_role_name};
use iota::test_scenario as ts;

// ===== Tests for Capability creation via RoleMap =====

#[test]
fun test_capability_created_with_correct_field_values() {
    let admin_user = @0xAD;
    let mut scenario = ts::begin(admin_user);

    let (mut role_map, admin_cap, security_vault_id) = test_utils::create_test_role_map(
        ts::ctx(&mut scenario),
    );

    // Verify capability has correct role
    assert!(admin_cap.role() == &initial_admin_role_name(), 0);
    // Verify capability has correct security_vault_id
    assert!(admin_cap.security_vault_id() == security_vault_id, 1);
    // Initial admin capability should have no address restriction
    assert!(admin_cap.issued_to().is_none(), 2);
    // Initial admin capability should have no validity restrictions
    assert!(admin_cap.valid_from().is_none(), 3);
    assert!(admin_cap.valid_until().is_none(), 4);

    // Cleanup
    role_map.destroy_capability(admin_cap);
    ts::end(scenario);
}

#[test]
fun test_has_role_returns_correct_values() {
    let admin_user = @0xAD;
    let mut scenario = ts::begin(admin_user);

    let (mut role_map, admin_cap, _security_vault_id) = test_utils::create_test_role_map(
        ts::ctx(&mut scenario),
    );

    // Verify has_role returns true for matching role
    assert!(admin_cap.has_role(&initial_admin_role_name()), 0);
    // Verify has_role returns false for non-matching role
    assert!(!admin_cap.has_role(&b"User".to_string()), 0);

    // Cleanup
    role_map.destroy_capability(admin_cap);
    ts::end(scenario);
}

// ===== Tests for Capability with issued_to restriction =====

#[test]
fun test_capability_issued_to_specific_address() {
    let admin_user = @0xAD;
    let target_user = @0xBEEF;
    let mut scenario = ts::begin(admin_user);

    let (mut role_map, admin_cap, _security_vault_id) = test_utils::create_test_role_map(
        ts::ctx(&mut scenario),
    );

    ts::next_tx(&mut scenario, admin_user);
    let clock = iota::clock::create_for_testing(ts::ctx(&mut scenario));

    // Create capability for specific address
    let user_cap = role_map.new_capability_for_address(
        &admin_cap,
        &initial_admin_role_name(),
        target_user,
        std::option::none(),
        &clock,
        ts::ctx(&mut scenario),
    );

    // Verify issued_to is set correctly
    assert!(user_cap.issued_to().is_some(), 0);
    assert!(*user_cap.issued_to().borrow() == target_user, 1);

    // Cleanup
    iota::clock::destroy_for_testing(clock);
    role_map.destroy_capability(admin_cap);
    role_map.destroy_capability(user_cap);
    ts::end(scenario);
}

// ===== Tests for Capability validity period =====

#[test]
fun test_capability_valid_from_and_valid_until() {
    let admin_user = @0xAD;
    let mut scenario = ts::begin(admin_user);

    let (mut role_map, admin_cap, _security_vault_id) = test_utils::create_test_role_map(
        ts::ctx(&mut scenario),
    );

    ts::next_tx(&mut scenario, admin_user);
    let clock = iota::clock::create_for_testing(ts::ctx(&mut scenario));

    let valid_from_ts = 1000u64;
    let valid_until_ts = 2000u64;

    // Create capability with validity period
    let timed_cap = role_map.new_capability(
        &admin_cap,
        &initial_admin_role_name(),
        std::option::none(),
        std::option::some(valid_from_ts),
        std::option::some(valid_until_ts),
        &clock,
        ts::ctx(&mut scenario),
    );

    // Verify validity period is set correctly
    assert!(timed_cap.valid_from().is_some(), 0);
    assert!(*timed_cap.valid_from().borrow() == valid_from_ts, 1);
    assert!(timed_cap.valid_until().is_some(), 2);
    assert!(*timed_cap.valid_until().borrow() == valid_until_ts, 3);

    // Cleanup
    iota::clock::destroy_for_testing(clock);
    role_map.destroy_capability(admin_cap);
    role_map.destroy_capability(timed_cap);
    ts::end(scenario);
}

// ===== Tests for is_valid_for_timestamp =====

#[test]
fun test_is_valid_for_timestamp_no_restrictions() {
    let admin_user = @0xAD;
    let mut scenario = ts::begin(admin_user);

    let (mut role_map, admin_cap, _security_vault_id) = test_utils::create_test_role_map(
        ts::ctx(&mut scenario),
    );

    // Capability without restrictions should be valid for any timestamp
    assert!(admin_cap.is_valid_for_timestamp(0), 0);
    assert!(admin_cap.is_valid_for_timestamp(1000), 1);
    assert!(admin_cap.is_valid_for_timestamp(999999999), 2);

    // Cleanup
    role_map.destroy_capability(admin_cap);
    ts::end(scenario);
}

#[test]
fun test_is_valid_for_timestamp_with_valid_from() {
    let admin_user = @0xAD;
    let mut scenario = ts::begin(admin_user);

    let (mut role_map, admin_cap, _security_vault_id) = test_utils::create_test_role_map(
        ts::ctx(&mut scenario),
    );

    ts::next_tx(&mut scenario, admin_user);
    let clock = iota::clock::create_for_testing(ts::ctx(&mut scenario));

    let valid_from_ts = 1000u64;

    // Create capability with valid_from only
    let timed_cap = role_map.new_capability(
        &admin_cap,
        &initial_admin_role_name(),
        std::option::none(),
        std::option::some(valid_from_ts),
        std::option::none(),
        &clock,
        ts::ctx(&mut scenario),
    );

    // Should be invalid before valid_from
    assert!(!timed_cap.is_valid_for_timestamp(999), 0);
    // Should be valid at valid_from (inclusive)
    assert!(timed_cap.is_valid_for_timestamp(1000), 1);
    // Should be valid after valid_from
    assert!(timed_cap.is_valid_for_timestamp(1001), 2);

    // Cleanup
    iota::clock::destroy_for_testing(clock);
    role_map.destroy_capability(admin_cap);
    role_map.destroy_capability(timed_cap);
    ts::end(scenario);
}

#[test]
fun test_is_valid_for_timestamp_with_valid_until() {
    let admin_user = @0xAD;
    let mut scenario = ts::begin(admin_user);

    let (mut role_map, admin_cap, _security_vault_id) = test_utils::create_test_role_map(
        ts::ctx(&mut scenario),
    );

    ts::next_tx(&mut scenario, admin_user);
    let clock = iota::clock::create_for_testing(ts::ctx(&mut scenario));

    let valid_until_ts = 2000u64;

    // Create capability with valid_until only
    let timed_cap = role_map.new_capability(
        &admin_cap,
        &initial_admin_role_name(),
        std::option::none(),
        std::option::none(),
        std::option::some(valid_until_ts),
        &clock,
        ts::ctx(&mut scenario),
    );

    // Should be valid before valid_until
    assert!(timed_cap.is_valid_for_timestamp(1999), 0);
    // Should be invalid at valid_until (exclusive)
    assert!(!timed_cap.is_valid_for_timestamp(2000), 1);
    // Should be invalid after valid_until
    assert!(!timed_cap.is_valid_for_timestamp(2001), 2);

    // Cleanup
    iota::clock::destroy_for_testing(clock);
    role_map.destroy_capability(admin_cap);
    role_map.destroy_capability(timed_cap);
    ts::end(scenario);
}

#[test]
fun test_is_valid_for_timestamp_with_both_restrictions() {
    let admin_user = @0xAD;
    let mut scenario = ts::begin(admin_user);

    let (mut role_map, admin_cap, _security_vault_id) = test_utils::create_test_role_map(
        ts::ctx(&mut scenario),
    );

    ts::next_tx(&mut scenario, admin_user);
    let clock = iota::clock::create_for_testing(ts::ctx(&mut scenario));

    let valid_from_ts = 1000u64;
    let valid_until_ts = 2000u64;

    // Create capability with both valid_from and valid_until
    let timed_cap = role_map.new_capability(
        &admin_cap,
        &initial_admin_role_name(),
        std::option::none(),
        std::option::some(valid_from_ts),
        std::option::some(valid_until_ts),
        &clock,
        ts::ctx(&mut scenario),
    );

    // Should be invalid before valid_from
    assert!(!timed_cap.is_valid_for_timestamp(999), 0);
    // Should be valid at valid_from (inclusive)
    assert!(timed_cap.is_valid_for_timestamp(1000), 1);
    // Should be valid between valid_from and valid_until
    assert!(timed_cap.is_valid_for_timestamp(1500), 2);
    // Should be invalid at valid_until (exclusive)
    assert!(!timed_cap.is_valid_for_timestamp(2000), 3);
    // Should be invalid after valid_until
    assert!(!timed_cap.is_valid_for_timestamp(2001), 4);

    // Cleanup
    iota::clock::destroy_for_testing(clock);
    role_map.destroy_capability(admin_cap);
    role_map.destroy_capability(timed_cap);
    ts::end(scenario);
}

// ===== Tests for is_currently_valid =====

#[test]
fun test_is_currently_valid_no_restrictions() {
    let admin_user = @0xAD;
    let mut scenario = ts::begin(admin_user);

    let (mut role_map, admin_cap, _security_vault_id) = test_utils::create_test_role_map(
        ts::ctx(&mut scenario),
    );

    ts::next_tx(&mut scenario, admin_user);
    let clock = iota::clock::create_for_testing(ts::ctx(&mut scenario));

    // Capability without restrictions should be currently valid
    assert!(admin_cap.is_currently_valid(&clock), 0);

    // Cleanup
    iota::clock::destroy_for_testing(clock);
    role_map.destroy_capability(admin_cap);
    ts::end(scenario);
}

#[test]
fun test_is_currently_valid_within_validity_period() {
    let admin_user = @0xAD;
    let mut scenario = ts::begin(admin_user);

    let (mut role_map, admin_cap, _security_vault_id) = test_utils::create_test_role_map(
        ts::ctx(&mut scenario),
    );

    ts::next_tx(&mut scenario, admin_user);
    let mut clock = iota::clock::create_for_testing(ts::ctx(&mut scenario));
    // Set clock to 1500 seconds (1500000 ms)
    iota::clock::set_for_testing(&mut clock, 1500000);

    let valid_from_ts = 1000u64;
    let valid_until_ts = 2000u64;

    // Create capability with validity period
    let timed_cap = role_map.new_capability(
        &admin_cap,
        &initial_admin_role_name(),
        std::option::none(),
        std::option::some(valid_from_ts),
        std::option::some(valid_until_ts),
        &clock,
        ts::ctx(&mut scenario),
    );

    // Should be currently valid (clock is at 1500s, within [1000, 2000))
    assert!(timed_cap.is_currently_valid(&clock), 0);

    // Cleanup
    iota::clock::destroy_for_testing(clock);
    role_map.destroy_capability(admin_cap);
    role_map.destroy_capability(timed_cap);
    ts::end(scenario);
}

#[test]
fun test_is_currently_valid_before_validity_period() {
    let admin_user = @0xAD;
    let mut scenario = ts::begin(admin_user);

    let (mut role_map, admin_cap, _security_vault_id) = test_utils::create_test_role_map(
        ts::ctx(&mut scenario),
    );

    ts::next_tx(&mut scenario, admin_user);
    let mut clock = iota::clock::create_for_testing(ts::ctx(&mut scenario));
    // Set clock to 500 seconds (500000 ms)
    iota::clock::set_for_testing(&mut clock, 500000);

    let valid_from_ts = 1000u64;
    let valid_until_ts = 2000u64;

    // Create capability with validity period
    let timed_cap = role_map.new_capability(
        &admin_cap,
        &initial_admin_role_name(),
        std::option::none(),
        std::option::some(valid_from_ts),
        std::option::some(valid_until_ts),
        &clock,
        ts::ctx(&mut scenario),
    );

    // Should not be currently valid (clock is at 500s, before 1000s)
    assert!(!timed_cap.is_currently_valid(&clock), 0);

    // Cleanup
    iota::clock::destroy_for_testing(clock);
    role_map.destroy_capability(admin_cap);
    role_map.destroy_capability(timed_cap);
    ts::end(scenario);
}

#[test]
fun test_is_currently_valid_after_validity_period() {
    let admin_user = @0xAD;
    let mut scenario = ts::begin(admin_user);

    let (mut role_map, admin_cap, _security_vault_id) = test_utils::create_test_role_map(
        ts::ctx(&mut scenario),
    );

    ts::next_tx(&mut scenario, admin_user);
    let mut clock = iota::clock::create_for_testing(ts::ctx(&mut scenario));
    // Set clock to 2500 seconds (2500000 ms)
    iota::clock::set_for_testing(&mut clock, 2500000);

    let valid_from_ts = 1000u64;
    let valid_until_ts = 2000u64;

    // Create capability with validity period
    let timed_cap = role_map.new_capability(
        &admin_cap,
        &initial_admin_role_name(),
        std::option::none(),
        std::option::some(valid_from_ts),
        std::option::some(valid_until_ts),
        &clock,
        ts::ctx(&mut scenario),
    );

    // Should not be currently valid (clock is at 2500s, after 2000s)
    assert!(!timed_cap.is_currently_valid(&clock), 0);

    // Cleanup
    iota::clock::destroy_for_testing(clock);
    role_map.destroy_capability(admin_cap);
    role_map.destroy_capability(timed_cap);
    ts::end(scenario);
}

// ===== Tests for capability with all restrictions =====

#[test]
fun test_capability_with_all_restrictions() {
    let admin_user = @0xAD;
    let target_user = @0xCAFE;
    let mut scenario = ts::begin(admin_user);

    let (mut role_map, admin_cap, security_vault_id) = test_utils::create_test_role_map(
        ts::ctx(&mut scenario),
    );

    ts::next_tx(&mut scenario, admin_user);
    let clock = iota::clock::create_for_testing(ts::ctx(&mut scenario));

    let valid_from_ts = 1000u64;
    let valid_until_ts = 2000u64;

    // Create capability with all restrictions
    let restricted_cap = role_map.new_capability(
        &admin_cap,
        &initial_admin_role_name(),
        std::option::some(target_user),
        std::option::some(valid_from_ts),
        std::option::some(valid_until_ts),
        &clock,
        ts::ctx(&mut scenario),
    );

    // Verify all fields
    assert!(restricted_cap.role() == &initial_admin_role_name(), 0);
    assert!(restricted_cap.security_vault_id() == security_vault_id, 1);
    assert!(restricted_cap.issued_to().is_some(), 2);
    assert!(*restricted_cap.issued_to().borrow() == target_user, 3);
    assert!(restricted_cap.valid_from().is_some(), 4);
    assert!(*restricted_cap.valid_from().borrow() == valid_from_ts, 5);
    assert!(restricted_cap.valid_until().is_some(), 6);
    assert!(*restricted_cap.valid_until().borrow() == valid_until_ts, 7);

    // Cleanup
    iota::clock::destroy_for_testing(clock);
    role_map.destroy_capability(admin_cap);
    role_map.destroy_capability(restricted_cap);
    ts::end(scenario);
}
