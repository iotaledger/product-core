// Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// This module provides tests for the timelock module
#[test_only]
module tf_components::timelock_tests;

use iota::{clock, test_scenario::{Self as ts, ctx}};
use tf_components::timelock;

const ADMIN_ADDRESS: address = @0x01;

#[test]
public fun test_new_unlock_at() {
    let mut ts = ts::begin(ADMIN_ADDRESS);

    let ctx = ts.ctx();

    let mut clock = clock::create_for_testing(ctx);
    clock::set_for_testing(&mut clock, 1000000);

    let lock = timelock::unlock_at(1001, &clock);

    assert!(timelock::is_unlock_at(&lock));
    assert!(timelock::get_unlock_time(&lock) == std::option::some(1001));
    assert!(timelock::is_timelocked(&lock, &clock));

    // Advance time by setting a new timestamp
    clock::increment_for_testing(&mut clock, 1000);

    assert!(!timelock::is_timelocked(&lock, &clock));

    timelock::destroy(lock, &clock);
    clock::destroy_for_testing(clock);

    ts.end();
}

#[test]
#[expected_failure(abort_code = timelock::EPastTimestamp)]
public fun test_new_unlock_at_past_time() {
    let mut ts = ts::begin(ADMIN_ADDRESS);
    let ctx = ts.ctx();

    let mut clock = clock::create_for_testing(ctx);
    clock::set_for_testing(&mut clock, 1000000);

    // Try to create a timelock with a timestamp in the past
    let lock = timelock::unlock_at(999, &clock);

    // This should never be reached
    timelock::destroy(lock, &clock);
    clock::destroy_for_testing(clock);

    ts.end();
}

#[test]
public fun test_until_destroyed() {
    let mut ts = ts::begin(ADMIN_ADDRESS);
    let ctx = ts.ctx();

    let mut clock = clock::create_for_testing(ctx);
    clock::set_for_testing(&mut clock, 1000000);

    let lock = timelock::until_destroyed();

    assert!(timelock::is_until_destroyed(&lock));
    assert!(!timelock::is_unlock_at(&lock));
    assert!(timelock::get_unlock_time(&lock) == std::option::none());

    // UntilDestroyed is always timelocked
    assert!(timelock::is_timelocked(&lock, &clock));

    // Even after a long time
    clock::increment_for_testing(&mut clock, 1000000);
    assert!(timelock::is_timelocked(&lock, &clock));

    // UntilDestroyed can always be destroyed without error
    timelock::destroy(lock, &clock);
    clock::destroy_for_testing(clock);

    ts.end();
}

#[test]
public fun test_none_lock() {
    let mut ts = ts::begin(ADMIN_ADDRESS);
    let ctx = ts.ctx();

    let mut clock = clock::create_for_testing(ctx);
    clock::set_for_testing(&mut clock, 1000000);

    let lock = timelock::none();

    assert!(!timelock::is_until_destroyed(&lock));
    assert!(!timelock::is_unlock_at(&lock));
    assert!(timelock::get_unlock_time(&lock) == std::option::none());

    // None is never timelocked
    assert!(!timelock::is_timelocked(&lock, &clock));

    // None can always be destroyed without error
    timelock::destroy(lock, &clock);
    clock::destroy_for_testing(clock);

    ts.end();
}

#[test]
#[expected_failure(abort_code = timelock::ETimelockNotExpired)]
public fun test_destroy_locked_timelock() {
    let mut ts = ts::begin(ADMIN_ADDRESS);
    let ctx = ts.ctx();

    let mut clock = clock::create_for_testing(ctx);
    clock::set_for_testing(&mut clock, 1000000);

    // Create a timelock that unlocks at time 2000
    let lock = timelock::unlock_at(2000, &clock);

    // Try to destroy it before it's unlocked
    // This should fail with ETimelockNotExpired
    timelock::destroy(lock, &clock);

    // These should never be reached
    clock::destroy_for_testing(clock);
    ts.end();
}

#[test]
public fun test_is_timelocked_unlock_at() {
    let mut ts = ts::begin(ADMIN_ADDRESS);
    let ctx = ts.ctx();

    let mut clock = clock::create_for_testing(ctx);
    clock::set_for_testing(&mut clock, 1000000);

    // Create different types of locks
    let unlock_at_lock = timelock::unlock_at(2000, &clock);
    let until_destroyed_lock = timelock::until_destroyed();
    let none_lock = timelock::none();

    // Test is_timelocked_unlock_at
    assert!(timelock::is_timelocked_unlock_at(&unlock_at_lock, &clock));
    assert!(!timelock::is_timelocked_unlock_at(&until_destroyed_lock, &clock));
    assert!(!timelock::is_timelocked_unlock_at(&none_lock, &clock));

    // Advance time past unlock time
    clock::increment_for_testing(&mut clock, 1000000);

    // Now the unlock_at lock should not be timelocked
    assert!(!timelock::is_timelocked_unlock_at(&unlock_at_lock, &clock));

    // Clean up
    timelock::destroy(unlock_at_lock, &clock);
    timelock::destroy(until_destroyed_lock, &clock);
    timelock::destroy(none_lock, &clock);
    clock::destroy_for_testing(clock);

    ts.end();
}

#[test]
public fun test_is_valid_period() {
    // Test valid periods
    assert!(timelock::is_valid_period(1001, 1000));
    assert!(timelock::is_valid_period(2000, 1000));

    // Test invalid periods
    assert!(!timelock::is_valid_period(1000, 1000)); // Equal time
    assert!(!timelock::is_valid_period(999, 1000)); // Past time
}

#[test]
public fun test_edge_cases() {
    let mut ts = ts::begin(ADMIN_ADDRESS);
    let ctx = ts.ctx();

    let mut clock = clock::create_for_testing(ctx);
    clock::set_for_testing(&mut clock, 1000000);

    // Test with time just one second in the future
    let one_second_future = timelock::unlock_at(1001, &clock);
    assert!(timelock::is_timelocked(&one_second_future, &clock));
    clock::set_for_testing(&mut clock, 1001000);
    assert!(!timelock::is_timelocked(&one_second_future, &clock));

    // Test with time exactly at the current time boundary
    clock::set_for_testing(&mut clock, 2000000);
    let exact_current_time = timelock::unlock_at(2001, &clock);
    assert!(timelock::is_timelocked(&exact_current_time, &clock));
    clock::set_for_testing(&mut clock, 2001000);
    assert!(!timelock::is_timelocked(&exact_current_time, &clock));

    // Clean up
    timelock::destroy(one_second_future, &clock);
    timelock::destroy(exact_current_time, &clock);
    clock::destroy_for_testing(clock);

    ts.end();
}

// ===== UnlockAtMs tests =====

#[test]
public fun test_new_unlock_at_ms() {
    let mut ts = ts::begin(ADMIN_ADDRESS);
    let ctx = ts.ctx();

    let mut clock = clock::create_for_testing(ctx);
    clock::set_for_testing(&mut clock, 1000000);

    // Create a lock that unlocks at 1001000 ms (1 second after current time)
    let lock = timelock::unlock_at_ms(1001000, &clock);

    assert!(timelock::is_unlock_at_ms(&lock));
    assert!(!timelock::is_unlock_at(&lock));
    assert!(!timelock::is_until_destroyed(&lock));
    assert!(!timelock::is_none(&lock));
    assert!(!timelock::is_infinite(&lock));
    assert!(timelock::get_unlock_time_ms(&lock) == std::option::some(1001000));
    assert!(timelock::get_unlock_time(&lock) == std::option::none());
    assert!(timelock::is_timelocked(&lock, &clock));

    // Advance time by 1 second (1000 ms)
    clock::increment_for_testing(&mut clock, 1000);

    // Should no longer be timelocked
    assert!(!timelock::is_timelocked(&lock, &clock));

    timelock::destroy(lock, &clock);
    clock::destroy_for_testing(clock);

    ts.end();
}

#[test]
#[expected_failure(abort_code = timelock::EPastTimestamp)]
public fun test_new_unlock_at_ms_past_time() {
    let mut ts = ts::begin(ADMIN_ADDRESS);
    let ctx = ts.ctx();

    let mut clock = clock::create_for_testing(ctx);
    clock::set_for_testing(&mut clock, 1000000);

    // Try to create a timelock with a timestamp in the past (999000 ms < 1000000 ms)
    let lock = timelock::unlock_at_ms(999000, &clock);

    // This should never be reached
    timelock::destroy(lock, &clock);
    clock::destroy_for_testing(clock);

    ts.end();
}

#[test]
#[expected_failure(abort_code = timelock::ETimelockNotExpired)]
public fun test_destroy_locked_timelock_ms() {
    let mut ts = ts::begin(ADMIN_ADDRESS);
    let ctx = ts.ctx();

    let mut clock = clock::create_for_testing(ctx);
    clock::set_for_testing(&mut clock, 1000000);

    // Create a timelock that unlocks at time 2000000 ms
    let lock = timelock::unlock_at_ms(2000000, &clock);

    // Try to destroy it before it's unlocked
    // This should fail with ETimelockNotExpired
    timelock::destroy(lock, &clock);

    // These should never be reached
    clock::destroy_for_testing(clock);
    ts.end();
}

#[test]
public fun test_is_timelocked_unlock_at_ms() {
    let mut ts = ts::begin(ADMIN_ADDRESS);
    let ctx = ts.ctx();

    let mut clock = clock::create_for_testing(ctx);
    clock::set_for_testing(&mut clock, 1000000);

    // Create an UnlockAtMs lock
    let unlock_at_ms_lock = timelock::unlock_at_ms(2000000, &clock);

    // Test is_timelocked_unlock_at (should work for both UnlockAt and UnlockAtMs)
    assert!(timelock::is_timelocked_unlock_at(&unlock_at_ms_lock, &clock));

    // Advance time past unlock time
    clock::set_for_testing(&mut clock, 2000001);

    // Now the unlock_at_ms lock should not be timelocked
    assert!(!timelock::is_timelocked_unlock_at(&unlock_at_ms_lock, &clock));

    // Clean up
    timelock::destroy(unlock_at_ms_lock, &clock);
    clock::destroy_for_testing(clock);

    ts.end();
}

#[test]
public fun test_is_valid_period_ms() {
    // Test valid periods
    assert!(timelock::is_valid_period_ms(1001, 1000));
    assert!(timelock::is_valid_period_ms(2000, 1000));

    // Test invalid periods
    assert!(!timelock::is_valid_period_ms(1000, 1000)); // Equal time
    assert!(!timelock::is_valid_period_ms(999, 1000)); // Past time
}

#[test]
public fun test_unlock_at_ms_edge_cases() {
    let mut ts = ts::begin(ADMIN_ADDRESS);
    let ctx = ts.ctx();

    let mut clock = clock::create_for_testing(ctx);
    clock::set_for_testing(&mut clock, 1000000);

    // Test with time just one millisecond in the future
    let one_ms_future = timelock::unlock_at_ms(1000001, &clock);
    assert!(timelock::is_timelocked(&one_ms_future, &clock));
    clock::set_for_testing(&mut clock, 1000001);
    assert!(!timelock::is_timelocked(&one_ms_future, &clock));

    // Test with larger timestamp values
    clock::set_for_testing(&mut clock, 1000000000000); // ~2001 in Unix timestamp ms
    let large_timestamp = timelock::unlock_at_ms(1000000000001, &clock);
    assert!(timelock::is_timelocked(&large_timestamp, &clock));
    clock::set_for_testing(&mut clock, 1000000000001);
    assert!(!timelock::is_timelocked(&large_timestamp, &clock));

    // Clean up
    timelock::destroy(one_ms_future, &clock);
    timelock::destroy(large_timestamp, &clock);
    clock::destroy_for_testing(clock);

    ts.end();
}

// ===== Infinite lock tests =====

#[test]

public fun test_infinite_lock() {
    let mut ts = ts::begin(ADMIN_ADDRESS);
    let ctx = ts.ctx();

    let mut clock = clock::create_for_testing(ctx);
    clock::set_for_testing(&mut clock, 1000000);

    let lock = timelock::infinite();

    assert!(timelock::is_infinite(&lock));
    assert!(!timelock::is_until_destroyed(&lock));
    assert!(!timelock::is_unlock_at(&lock));
    assert!(!timelock::is_unlock_at_ms(&lock));
    assert!(!timelock::is_none(&lock));
    assert!(timelock::get_unlock_time(&lock) == std::option::none());
    assert!(timelock::get_unlock_time_ms(&lock) == std::option::none());

    // Infinite is always timelocked
    assert!(timelock::is_timelocked(&lock, &clock));

    // Even after a very long time, Infinite lock remains locked
    clock::increment_for_testing(&mut clock, 1000000000000);
    assert!(timelock::is_timelocked(&lock, &clock));

    // Infinite should not be detected as timelocked_unlock_at
    assert!(!timelock::is_timelocked_unlock_at(&lock, &clock));

    // Note: Infinite lock cannot be destroyed (tested separately)
    // Therefore we wrw using a test-only destroy here
    timelock::destroy_for_testing(lock);
    clock::destroy_for_testing(clock); 
    ts.end();
}

#[test]
#[expected_failure(abort_code = timelock::ETimelockNotExpired)]
public fun test_destroy_infinite_lock() {
    let mut ts = ts::begin(ADMIN_ADDRESS);
    let ctx = ts.ctx();

    let mut clock = clock::create_for_testing(ctx);
    clock::set_for_testing(&mut clock, 1000000);

    let lock = timelock::infinite();

    // Try to destroy an Infinite lock
    // This should always fail with ETimelockNotExpired
    timelock::destroy(lock, &clock);

    // These should never be reached
    clock::destroy_for_testing(clock);
    ts.end();
}

#[test]
#[expected_failure(abort_code = timelock::ETimelockNotExpired)]
public fun test_destroy_infinite_lock_after_long_time() {
    let mut ts = ts::begin(ADMIN_ADDRESS);
    let ctx = ts.ctx();

    let mut clock = clock::create_for_testing(ctx);
    clock::set_for_testing(&mut clock, 1000000);

    let lock = timelock::infinite();

    // Even after a very long time, Infinite lock cannot be destroyed
    clock::increment_for_testing(&mut clock, 1000000000000);

    // This should still fail
    timelock::destroy(lock, &clock);

    // These should never be reached
    clock::destroy_for_testing(clock);
    ts.end();
}

#[test]
#[expected_failure(abort_code = timelock::ETimelockNotExpired)]
public fun test_infinite_vs_until_destroyed() {
    let mut ts = ts::begin(ADMIN_ADDRESS);
    let ctx = ts.ctx();

    let mut clock = clock::create_for_testing(ctx);
    clock::set_for_testing(&mut clock, 1000000);

    let infinite_lock = timelock::infinite();
    let until_destroyed_lock = timelock::until_destroyed();

    // Both are always timelocked
    assert!(timelock::is_timelocked(&infinite_lock, &clock));
    assert!(timelock::is_timelocked(&until_destroyed_lock, &clock));

    // But they have different type checks
    assert!(timelock::is_infinite(&infinite_lock));
    assert!(!timelock::is_until_destroyed(&infinite_lock));
    assert!(!timelock::is_infinite(&until_destroyed_lock));
    assert!(timelock::is_until_destroyed(&until_destroyed_lock));

    // Key difference: UntilDestroyed can be destroyed, Infinite cannot
    // UntilDestroyed allows the locked object to be destroyed
    timelock::destroy(until_destroyed_lock, &clock);

    // This should fail with ETimelockNotExpired
    timelock::destroy(infinite_lock, &clock);
    
    // These should never be reached
    clock::destroy_for_testing(clock);
    ts.end();
}

#[test]
public fun test_all_lock_types_type_checks() {
    let mut ts = ts::begin(ADMIN_ADDRESS);
    let ctx = ts.ctx();

    let mut clock = clock::create_for_testing(ctx);
    clock::set_for_testing(&mut clock, 1000000);

    let unlock_at_lock = timelock::unlock_at(2000, &clock);
    let unlock_at_ms_lock = timelock::unlock_at_ms(2000000, &clock);
    let until_destroyed_lock = timelock::until_destroyed();
    let none_lock = timelock::none();
    let infinite_lock = timelock::infinite();

    // Test is_unlock_at
    assert!(timelock::is_unlock_at(&unlock_at_lock));
    assert!(!timelock::is_unlock_at(&unlock_at_ms_lock));
    assert!(!timelock::is_unlock_at(&until_destroyed_lock));
    assert!(!timelock::is_unlock_at(&none_lock));
    assert!(!timelock::is_unlock_at(&infinite_lock));

    // Test is_unlock_at_ms
    assert!(!timelock::is_unlock_at_ms(&unlock_at_lock));
    assert!(timelock::is_unlock_at_ms(&unlock_at_ms_lock));
    assert!(!timelock::is_unlock_at_ms(&until_destroyed_lock));
    assert!(!timelock::is_unlock_at_ms(&none_lock));
    assert!(!timelock::is_unlock_at_ms(&infinite_lock));

    // Test is_until_destroyed
    assert!(!timelock::is_until_destroyed(&unlock_at_lock));
    assert!(!timelock::is_until_destroyed(&unlock_at_ms_lock));
    assert!(timelock::is_until_destroyed(&until_destroyed_lock));
    assert!(!timelock::is_until_destroyed(&none_lock));
    assert!(!timelock::is_until_destroyed(&infinite_lock));

    // Test is_none
    assert!(!timelock::is_none(&unlock_at_lock));
    assert!(!timelock::is_none(&unlock_at_ms_lock));
    assert!(!timelock::is_none(&until_destroyed_lock));
    assert!(timelock::is_none(&none_lock));
    assert!(!timelock::is_none(&infinite_lock));

    // Test is_infinite
    assert!(!timelock::is_infinite(&unlock_at_lock));
    assert!(!timelock::is_infinite(&unlock_at_ms_lock));
    assert!(!timelock::is_infinite(&until_destroyed_lock));
    assert!(!timelock::is_infinite(&none_lock));
    assert!(timelock::is_infinite(&infinite_lock));

    // Clean up destroyable locks
    clock::set_for_testing(&mut clock, 3000000);
    timelock::destroy(unlock_at_lock, &clock);
    timelock::destroy(unlock_at_ms_lock, &clock);
    timelock::destroy(until_destroyed_lock, &clock);
    timelock::destroy(none_lock, &clock);

    // Infinite lock can not be destroyed as usual, using test-only destroy instead
    timelock::destroy_for_testing(infinite_lock);
    
    // Cleanup
    clock::destroy_for_testing(clock);
    ts.end();    
}
