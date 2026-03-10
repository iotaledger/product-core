// Copyright (c) 2026 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
#[allow(lint(abort_without_constant))]
#[test_only]
module tf_components::role_map_tests;

use std::string::String;

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
        let (role_map, admin_cap) = role_map::new<_, String>(
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

        let role_admin_data = b"RoleAdmin role data".to_string();
        let cap_admin_data = b"CapAdmin role data".to_string();

        // Verify initial state - should only have the initial admin role
        assert!(role_map.size() == 1, 2);

        // Create RoleAdmin role
        role_map.create_role(
            &admin_cap,
            string::utf8(b"RoleAdmin"),
            vec_set::singleton(test_utils::manage_roles()),
            std::option::some(role_admin_data),
            &clock,
            ts::ctx(&mut scenario),
        );

        // Create CapAdmin role
        role_map.create_role(
            &admin_cap,
            string::utf8(b"CapAdmin"),
            vec_set::singleton(test_utils::manage_capabilities()),
            std::option::some(cap_admin_data),
            &clock,
            ts::ctx(&mut scenario),
        );

        // Verify both roles were created
        assert!(role_map.size() == 3, 3); // Initial admin + RoleAdmin + CapAdmin
        assert!(role_map.has_role(&b"RoleAdmin".to_string()), 4);
        assert!(role_map.has_role(&b"CapAdmin".to_string()), 5);
        assert!(role_map.get_role_data(&b"RoleAdmin".to_string()) == std::option::some(role_admin_data), 6);
        assert!(role_map.get_role_data(&b"CapAdmin".to_string()) == std::option::some(cap_admin_data), 7);

        iota::clock::destroy_for_testing(clock);
    };

    // Step 3: Admin updates RoleAdmin and CapAdmin roles
    ts::next_tx(&mut scenario, admin_user);
    {
        let clock = iota::clock::create_for_testing(ts::ctx(&mut scenario));

        let updated_role_admin_data = b"Updated RoleAdmin role data".to_string();
        let updated_cap_admin_data = b"Updated CapAdmin role data".to_string();

        // Update RoleAdmin role permissions and data - for simplicity, we swap the permissions to each other's permissions
        role_map.update_role(
            &admin_cap,
            &b"RoleAdmin".to_string(),
            vec_set::singleton(test_utils::manage_capabilities()),
            std::option::some(updated_role_admin_data),
            &clock,
            ts::ctx(&mut scenario),
        );

        // Update CapAdmin role - for simplicity, we swap the permissions to each other's permissions
        role_map.update_role(
            &admin_cap,
            &b"CapAdmin".to_string(),
            vec_set::singleton(test_utils::manage_roles()),
            std::option::some(updated_cap_admin_data),
            &clock,
            ts::ctx(&mut scenario),
        );

        // Verify both roles were updated
        assert!(role_map.get_role_data(&b"RoleAdmin".to_string()) == std::option::some(updated_role_admin_data), 8);
        assert!(role_map.get_role_data(&b"CapAdmin".to_string()) == std::option::some(updated_cap_admin_data), 9);
        assert!(role_map.get_role_permissions(&b"RoleAdmin".to_string()).contains(&test_utils::manage_capabilities()), 10);
        assert!(role_map.get_role_permissions(&b"CapAdmin".to_string()).contains(&test_utils::manage_roles()), 11);

        iota::clock::destroy_for_testing(clock);
    };

    transfer::public_transfer(admin_cap, admin_user);

    // Cleanup
    role_map.destroy();
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

    let (mut role_map, admin_cap) = role_map::new<_, String>(
        target_key,
        b"SuperAdmin".to_string(),
        empty_permissions,
        role_admin_permissions,
        capability_admin_permissions,
        ts::ctx(&mut scenario),
    );

    role_map.destroy_initial_admin_capability(admin_cap);

    role_map.destroy();
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
    role_map.destroy();
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

    role_map.update_role(
        &admin_cap,
        &initial_role,
        vec_set::singleton(test_utils::manage_roles()),
        std::option::none(),
        &clock,
        ts::ctx(&mut scenario),
    );

    iota::clock::destroy_for_testing(clock);
    role_map.destroy_initial_admin_capability(admin_cap);
    role_map.destroy();
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
        std::option::none(),
        &clock,
        ts::ctx(&mut scenario),
    );

    iota::clock::destroy_for_testing(clock);
    role_map.destroy_initial_admin_capability(admin_cap);
    role_map.destroy();
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
    role_map.destroy();
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
        std::option::none(),
        &clock,
        ts::ctx(&mut scenario),
    );

    iota::clock::destroy_for_testing(clock);
    role_map.destroy_initial_admin_capability(admin_cap);
    role_map.destroy_initial_admin_capability(second_admin_cap);
    role_map.destroy();
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
    role_map.destroy();
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
    let _second_admin_cap_id = second_admin_cap.id();

    // Destroy the first admin cap via explicit API
    role_map.destroy_initial_admin_capability(admin_cap);

    iota::clock::destroy_for_testing(clock);
    transfer::public_transfer(second_admin_cap, admin_user);
    role_map.destroy();
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
        std::option::none(),
        &clock,
        ts::ctx(&mut scenario),
    );

    assert!(role_map.revoked_capabilities().length() == 1, 0);
    assert!(role_map.revoked_capabilities().contains(admin_cap.id()), 1);

    // The revoked cap object can still be destroyed via normal destroy (no longer in initial_admin_cap_ids)
    role_map.destroy_capability(admin_cap);

    // After being destroyed, the admin_cap is not contained in the revoked_capabilities list anymore
    assert!(role_map.revoked_capabilities().length() == 0, 2);

    iota::clock::destroy_for_testing(clock);
    transfer::public_transfer(second_admin_cap, admin_user);
    role_map.destroy();
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

    role_map.destroy();
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

    // Initially the `revoked_capabilities` list must be empty
    assert!(role_map.revoked_capabilities().length() == 0, 0);

    // Revoke the only admin cap — seals the RoleMap
    role_map.revoke_initial_admin_capability(
        &admin_cap,
        admin_cap.id(),
        std::option::none(),
        &clock,
        ts::ctx(&mut scenario),
    );

    assert!(role_map.revoked_capabilities().length() == 1, 1);
    assert!(role_map.revoked_capabilities().contains(admin_cap.id()), 2);

    iota::clock::destroy_for_testing(clock);
    // The revoked cap can still be destroyed for cleanup
    role_map.destroy_capability(admin_cap);
    role_map.destroy();
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
    let _rotated_admin_cap_id = rotated_admin_cap.id();
    
    // Initially the `revoked_capabilities` list must be empty
    assert!(role_map.revoked_capabilities().length() == 0, 0);

    // Use the explicit API to revoke the old admin cap
    role_map.revoke_initial_admin_capability(
        &admin_cap,
        admin_cap.id(),
        std::option::none(),
        &clock,
        ts::ctx(&mut scenario),
    );

    assert!(role_map.revoked_capabilities().length() == 1, 1);
    assert!(role_map.revoked_capabilities().contains(admin_cap.id()), 2);

    iota::clock::destroy_for_testing(clock);
    role_map.destroy_capability(admin_cap);
    role_map.destroy();
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
        std::option::none(),
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
    role_map.destroy();
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
        std::option::none(),
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
        std::option::none(),
        &clock,
        ts::ctx(&mut scenario),
    );

    iota::clock::destroy_for_testing(clock);
    transfer::public_transfer(admin_cap, admin_user);
    role_map.destroy_capability(reader_cap);
    role_map.destroy();
    ts::end(scenario);
}
