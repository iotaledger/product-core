// Copyright (c) 2026 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[test_only]
module tf_components::role_map_tests;

use std::string::String;

use iota::test_scenario as ts;
use iota::vec_set;
use std::string;
use tf_components::core_test_utils as test_utils;
use tf_components::role_map;

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


    role_map.destroy_capability(admin_cap);

    // Cleanup
    ts::next_tx(&mut scenario, admin_user);
    ts::end(scenario);
}
