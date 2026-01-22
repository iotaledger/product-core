// Copyright (c) 2026 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[test_only]
module tf_components::role_map_tests;

use tf_components::role_map;
use tf_components::core_test_utils as test_utils;
use iota::test_scenario as ts;
use iota::vec_set;
use std::string;

#[test]
fun test_role_based_permission_delegation() {
    let (
        role_admin_permissions,
        capability_admin_permissions,
    ) = test_utils::get_admin_permissions();

    let admin_user = @0xAD;
    let mut scenario = ts::begin(admin_user);

    let security_vault_id = test_utils::fake_object_id_from_string(
        &b"This is a test Vault ID String".to_string(),
    );
    let initial_admin_role_name = b"SuperAdmin".to_string();

    // Step 1: admin_user creates the audit trail
    let (mut role_map, admin_cap) = {
        let (role_map, admin_cap) = role_map::new(
            security_vault_id,
            initial_admin_role_name,
            test_utils::super_admin_permissions(),
            role_admin_permissions,
            capability_admin_permissions,
            ts::ctx(&mut scenario),
        );

        // Verify admin capability was created with correct role and trail reference
        assert!(admin_cap.role() == initial_admin_role_name, 0);
        assert!(admin_cap.security_vault_id() == security_vault_id, 1);

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

    role_map.destroy_capability(admin_cap);

    // Cleanup
    ts::next_tx(&mut scenario, admin_user);
    ts::end(scenario);
}
