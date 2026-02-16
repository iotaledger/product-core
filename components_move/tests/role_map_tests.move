// Copyright (c) 2026 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
#[allow(lint(abort_without_constant))]
#[test_only]
module tf_components::role_map_tests;

use iota::{test_scenario as ts, vec_set};
use std::string;
use tf_components::{core_test_utils as test_utils, role_map};

#[test]
fun test_role_based_permission_delegation() {
    let (
        role_admin_permissions,
        capability_admin_permissions,
    ) = test_utils::get_admin_permissions();

    let admin_user = @0xAD;
    let mut scenario = ts::begin(admin_user);

    let target_key = test_utils::fake_object_id_from_string(
        &b"This is a test Vault ID String".to_string(),
    );
    let initial_admin_role_name = b"SuperAdmin".to_string();

    // Step 1: admin_user creates the audit trail
    let (mut role_map, admin_cap) = {
        let (role_map, admin_cap) = role_map::new(
            target_key,
            initial_admin_role_name,
            test_utils::super_admin_permissions(),
            role_admin_permissions,
            capability_admin_permissions,
            ts::ctx(&mut scenario),
        );

        // Verify admin capability was created with correct role and trail reference
        assert!(admin_cap.role() == initial_admin_role_name, 0);
        assert!(admin_cap.target_key() == target_key, 1);

        (role_map, admin_cap)
    };

    // Step 2: Admin creates RoleAdmin and CapAdmin roles
    ts::next_tx(&mut scenario, admin_user);
    {
        let clock = iota::clock::create_for_testing(ts::ctx(&mut scenario));

        // Verify initial state - should only have the initial admin role
        assert!(role_map.size() == 1, 2);

        // Create RoleAdmin role
        role_map.create_role(
            &admin_cap,
            string::utf8(b"RoleAdmin"),
            vec_set::singleton(test_utils::manage_roles()),
            &clock,
            ts::ctx(&mut scenario),
        );

        // Create CapAdmin role
        role_map.create_role(
            &admin_cap,
            string::utf8(b"CapAdmin"),
            vec_set::singleton(test_utils::manage_capabilities()),
            &clock,
            ts::ctx(&mut scenario),
        );

        // Verify both roles were created
        assert!(role_map.size() == 3, 3); // Initial admin + RoleAdmin + CapAdmin
        assert!(role_map.has_role(&string::utf8(b"RoleAdmin")), 4);
        assert!(role_map.has_role(&string::utf8(b"CapAdmin")), 5);

        iota::clock::destroy_for_testing(clock);
    };

    transfer::public_transfer(admin_cap, admin_user);

    // Cleanup
    ts::next_tx(&mut scenario, admin_user);
    ts::end(scenario);
}

#[test]
#[expected_failure(abort_code = role_map::EInitialAdminPermissionsInconsistent)]
fun test_new_fails_with_empty_initial_admin_permissions() {
    let (
        role_admin_permissions,
        capability_admin_permissions,
    ) = test_utils::get_admin_permissions();

    let admin_user = @0xAD;
    let mut scenario = ts::begin(admin_user);

    let target_key = test_utils::fake_object_id_from_string(
        &b"This is a test Vault ID String".to_string(),
    );

    let empty_permissions = vec_set::empty<test_utils::Permission>();

    let (mut role_map, admin_cap) = role_map::new(
        target_key,
        b"SuperAdmin".to_string(),
        empty_permissions,
        role_admin_permissions,
        capability_admin_permissions,
        ts::ctx(&mut scenario),
    );

    role_map.destroy_initial_admin_capability(admin_cap);

    ts::end(scenario);
}

#[test]
#[expected_failure(abort_code = role_map::EInitialAdminRoleCannotBeDeleted)]
fun test_delete_initial_admin_role_fails() {
    let admin_user = @0xAD;
    let mut scenario = ts::begin(admin_user);

    let (mut role_map, admin_cap, _target_key) = test_utils::create_test_role_map(
        ts::ctx(&mut scenario),
    );

    let clock = iota::clock::create_for_testing(ts::ctx(&mut scenario));
    let initial_role = test_utils::initial_admin_role_name();
    role_map.delete_role(
        &admin_cap,
        &initial_role,
        &clock,
        ts::ctx(&mut scenario),
    );

    iota::clock::destroy_for_testing(clock);
    role_map.destroy_initial_admin_capability(admin_cap);
    ts::end(scenario);
}

#[test]
#[expected_failure(abort_code = role_map::EInitialAdminPermissionsInconsistent)]
fun test_update_initial_admin_role_removing_required_permissions_fails() {
    let admin_user = @0xAD;
    let mut scenario = ts::begin(admin_user);

    let (mut role_map, admin_cap, _target_key) = test_utils::create_test_role_map(
        ts::ctx(&mut scenario),
    );

    let clock = iota::clock::create_for_testing(ts::ctx(&mut scenario));
    let initial_role = test_utils::initial_admin_role_name();

    role_map.update_role_permissions(
        &admin_cap,
        &initial_role,
        vec_set::singleton(test_utils::manage_roles()),
        &clock,
        ts::ctx(&mut scenario),
    );

    iota::clock::destroy_for_testing(clock);
    role_map.destroy_initial_admin_capability(admin_cap);
    ts::end(scenario);
}

// ===== Tests: normal revoke/destroy blocked for initial admin caps =====

#[test]
#[expected_failure(abort_code = role_map::EInitialAdminCapabilityMustBeExplicitlyDestroyed)]
fun test_revoke_initial_admin_capability_blocked_on_normal_revoke() {
    let admin_user = @0xAD;
    let mut scenario = ts::begin(admin_user);

    let (mut role_map, admin_cap, _target_key) = test_utils::create_test_role_map(
        ts::ctx(&mut scenario),
    );

    let clock = iota::clock::create_for_testing(ts::ctx(&mut scenario));
    role_map.revoke_capability(
        &admin_cap,
        admin_cap.id(),
        &clock,
        ts::ctx(&mut scenario),
    );

    iota::clock::destroy_for_testing(clock);
    role_map.destroy_initial_admin_capability(admin_cap);
    ts::end(scenario);
}

#[test]
#[expected_failure(abort_code = role_map::EInitialAdminCapabilityMustBeExplicitlyDestroyed)]
fun test_destroy_initial_admin_capability_blocked_on_normal_destroy() {
    let admin_user = @0xAD;
    let mut scenario = ts::begin(admin_user);

    let (mut role_map, admin_cap, _target_key) = test_utils::create_test_role_map(
        ts::ctx(&mut scenario),
    );

    role_map.destroy_capability(admin_cap);
    ts::end(scenario);
}

#[test]
#[expected_failure(abort_code = role_map::EInitialAdminCapabilityMustBeExplicitlyDestroyed)]
fun test_revoke_second_initial_admin_capability_blocked_on_normal_revoke() {
    let admin_user = @0xAD;
    let mut scenario = ts::begin(admin_user);

    let (mut role_map, admin_cap, _target_key) = test_utils::create_test_role_map(
        ts::ctx(&mut scenario),
    );

    let clock = iota::clock::create_for_testing(ts::ctx(&mut scenario));
    let initial_role = test_utils::initial_admin_role_name();

    // Issue a second admin cap
    let second_admin_cap = role_map.new_capability(
        &admin_cap,
        &initial_role,
        std::option::none(),
        std::option::none(),
        std::option::none(),
        &clock,
        ts::ctx(&mut scenario),
    );

    // Try to revoke the second one via normal revoke — should fail even with multiple admin caps
    role_map.revoke_capability(
        &admin_cap,
        second_admin_cap.id(),
        &clock,
        ts::ctx(&mut scenario),
    );

    iota::clock::destroy_for_testing(clock);
    role_map.destroy_initial_admin_capability(admin_cap);
    role_map.destroy_initial_admin_capability(second_admin_cap);
    ts::end(scenario);
}

#[test]
#[expected_failure(abort_code = role_map::EInitialAdminCapabilityMustBeExplicitlyDestroyed)]
fun test_destroy_second_initial_admin_capability_blocked_on_normal_destroy() {
    let admin_user = @0xAD;
    let mut scenario = ts::begin(admin_user);

    let (mut role_map, admin_cap, _target_key) = test_utils::create_test_role_map(
        ts::ctx(&mut scenario),
    );

    let clock = iota::clock::create_for_testing(ts::ctx(&mut scenario));
    let initial_role = test_utils::initial_admin_role_name();

    // Issue a second admin cap
    let second_admin_cap = role_map.new_capability(
        &admin_cap,
        &initial_role,
        std::option::none(),
        std::option::none(),
        std::option::none(),
        &clock,
        ts::ctx(&mut scenario),
    );

    // Try to destroy the second one via normal destroy — should fail
    iota::clock::destroy_for_testing(clock);
    transfer::public_transfer(admin_cap, admin_user);
    role_map.destroy_capability(second_admin_cap);
    ts::end(scenario);
}

#[test]
fun test_destroy_initial_admin_capability_works() {
    let admin_user = @0xAD;
    let mut scenario = ts::begin(admin_user);

    let (mut role_map, admin_cap, _target_key) = test_utils::create_test_role_map(
        ts::ctx(&mut scenario),
    );

    let clock = iota::clock::create_for_testing(ts::ctx(&mut scenario));
    let initial_role = test_utils::initial_admin_role_name();

    // Issue a second admin cap so we can destroy the first
    let second_admin_cap = role_map.new_capability(
        &admin_cap,
        &initial_role,
        std::option::none(),
        std::option::none(),
        std::option::none(),
        &clock,
        ts::ctx(&mut scenario),
    );
    let second_admin_cap_id = second_admin_cap.id();

    // Destroy the first admin cap via explicit API
    role_map.destroy_initial_admin_capability(admin_cap);

    assert!(role_map.issued_capabilities().size() == 1, 0);
    assert!(role_map.issued_capabilities().contains(&second_admin_cap_id), 1);

    iota::clock::destroy_for_testing(clock);
    transfer::public_transfer(second_admin_cap, admin_user);
    ts::end(scenario);
}

#[test]
fun test_revoke_initial_admin_capability_works() {
    let admin_user = @0xAD;
    let mut scenario = ts::begin(admin_user);

    let (mut role_map, admin_cap, _target_key) = test_utils::create_test_role_map(
        ts::ctx(&mut scenario),
    );

    let clock = iota::clock::create_for_testing(ts::ctx(&mut scenario));
    let initial_role = test_utils::initial_admin_role_name();

    // Issue a second admin cap
    let second_admin_cap = role_map.new_capability(
        &admin_cap,
        &initial_role,
        std::option::none(),
        std::option::none(),
        std::option::none(),
        &clock,
        ts::ctx(&mut scenario),
    );

    // Revoke the first admin cap via explicit API
    role_map.revoke_initial_admin_capability(
        &second_admin_cap,
        admin_cap.id(),
        &clock,
        ts::ctx(&mut scenario),
    );

    assert!(role_map.issued_capabilities().size() == 1, 0);
    assert!(role_map.issued_capabilities().contains(&second_admin_cap.id()), 1);

    // The revoked cap object can still be destroyed via normal destroy (no longer in initial_admin_cap_ids)
    role_map.destroy_capability(admin_cap);

    iota::clock::destroy_for_testing(clock);
    transfer::public_transfer(second_admin_cap, admin_user);
    ts::end(scenario);
}

#[test]
fun test_destroy_last_initial_admin_capability_seals_role_map() {
    let admin_user = @0xAD;
    let mut scenario = ts::begin(admin_user);

    let (mut role_map, admin_cap, _target_key) = test_utils::create_test_role_map(
        ts::ctx(&mut scenario),
    );

    // Destroy the only admin cap — seals the RoleMap
    role_map.destroy_initial_admin_capability(admin_cap);

    assert!(role_map.issued_capabilities().size() == 0, 0);

    ts::end(scenario);
}

#[test]
fun test_revoke_last_initial_admin_capability_seals_role_map() {
    let admin_user = @0xAD;
    let mut scenario = ts::begin(admin_user);

    let (mut role_map, admin_cap, _target_key) = test_utils::create_test_role_map(
        ts::ctx(&mut scenario),
    );

    let clock = iota::clock::create_for_testing(ts::ctx(&mut scenario));

    // Revoke the only admin cap — seals the RoleMap
    role_map.revoke_initial_admin_capability(
        &admin_cap,
        admin_cap.id(),
        &clock,
        ts::ctx(&mut scenario),
    );

    assert!(role_map.issued_capabilities().size() == 0, 0);

    iota::clock::destroy_for_testing(clock);
    // The revoked cap can still be destroyed for cleanup
    role_map.destroy_capability(admin_cap);
    ts::end(scenario);
}

#[test]
fun test_initial_admin_capability_rotation_works() {
    let admin_user = @0xAD;
    let mut scenario = ts::begin(admin_user);

    let (mut role_map, admin_cap, _target_key) = test_utils::create_test_role_map(
        ts::ctx(&mut scenario),
    );

    let clock = iota::clock::create_for_testing(ts::ctx(&mut scenario));
    let initial_role = test_utils::initial_admin_role_name();
    let rotated_admin_cap = role_map.new_capability(
        &admin_cap,
        &initial_role,
        std::option::none(),
        std::option::none(),
        std::option::none(),
        &clock,
        ts::ctx(&mut scenario),
    );
    let rotated_admin_cap_id = rotated_admin_cap.id();

    // Use the explicit API to revoke the old admin cap
    role_map.revoke_initial_admin_capability(
        &admin_cap,
        admin_cap.id(),
        &clock,
        ts::ctx(&mut scenario),
    );

    assert!(role_map.issued_capabilities().size() == 1, 0);
    assert!(role_map.issued_capabilities().contains(&rotated_admin_cap_id), 1);

    iota::clock::destroy_for_testing(clock);
    role_map.destroy_capability(admin_cap);
    transfer::public_transfer(rotated_admin_cap, admin_user);
    ts::end(scenario);
}

#[test]
#[expected_failure(abort_code = role_map::ECapabilityIsNotInitialAdmin)]
fun test_destroy_initial_admin_capability_rejects_non_admin_cap() {
    let admin_user = @0xAD;
    let mut scenario = ts::begin(admin_user);

    let (mut role_map, admin_cap, _target_key) = test_utils::create_test_role_map(
        ts::ctx(&mut scenario),
    );

    let clock = iota::clock::create_for_testing(ts::ctx(&mut scenario));

    // Create a non-admin role and capability
    role_map.create_role(
        &admin_cap,
        string::utf8(b"Reader"),
        vec_set::singleton(test_utils::manage_roles()),
        &clock,
        ts::ctx(&mut scenario),
    );
    let reader_cap = role_map.new_capability(
        &admin_cap,
        &string::utf8(b"Reader"),
        std::option::none(),
        std::option::none(),
        std::option::none(),
        &clock,
        ts::ctx(&mut scenario),
    );

    // Try to destroy reader cap via the explicit admin API — should fail
    iota::clock::destroy_for_testing(clock);
    transfer::public_transfer(admin_cap, admin_user);
    role_map.destroy_initial_admin_capability(reader_cap);
    ts::end(scenario);
}

#[test]
#[expected_failure(abort_code = role_map::ECapabilityIsNotInitialAdmin)]
fun test_revoke_initial_admin_capability_rejects_non_admin_cap() {
    let admin_user = @0xAD;
    let mut scenario = ts::begin(admin_user);

    let (mut role_map, admin_cap, _target_key) = test_utils::create_test_role_map(
        ts::ctx(&mut scenario),
    );

    let clock = iota::clock::create_for_testing(ts::ctx(&mut scenario));

    // Create a non-admin role and capability
    role_map.create_role(
        &admin_cap,
        string::utf8(b"Reader"),
        vec_set::singleton(test_utils::manage_roles()),
        &clock,
        ts::ctx(&mut scenario),
    );
    let reader_cap = role_map.new_capability(
        &admin_cap,
        &string::utf8(b"Reader"),
        std::option::none(),
        std::option::none(),
        std::option::none(),
        &clock,
        ts::ctx(&mut scenario),
    );

    // Try to revoke reader cap via the explicit admin API — should fail
    role_map.revoke_initial_admin_capability(
        &admin_cap,
        reader_cap.id(),
        &clock,
        ts::ctx(&mut scenario),
    );

    iota::clock::destroy_for_testing(clock);
    transfer::public_transfer(admin_cap, admin_user);
    role_map.destroy_capability(reader_cap);
    ts::end(scenario);
}
