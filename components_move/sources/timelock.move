// Copyright (c) 2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// # Timelock Unlock Condition Module
///
/// This module implements a timelock mechanism that restricts access to resources
/// until a specified time has passed.
module tf_components::timelock;

use iota::clock::{Self, Clock};

// ===== Errors =====
/// Error when attempting to create a timelock with a timestamp in the past
const EPastTimestamp: u64 = 0;
/// Error when attempting to destroy a timelock that is still locked
const ETimelockNotExpired: u64 = 1;

/// Represents different types of time-based locks that can be applied to
/// onchain objects.
public enum TimeLock has store {
    /// A lock that unlocks at a specific Unix timestamp (seconds since Unix epoch)
    UnlockAt(u32),
    /// Same as UnlockAt (unlocks at specific timestamp) but using milliseconds since Unix epoch
    UnlockAtMs(u64),
    /// A permanent lock that never unlocks until the locked object is destroyed (can't be used for `delete_lock`)
    UntilDestroyed,
    /// A lock that never unlocks (permanent lock)
    Infinite,
    /// No lock applied
    None,
}

/// Creates a new UnlockAt time lock that unlocks at a specific Unix timestamp.
public fun unlock_at(unix_time: u32, clock: &Clock): TimeLock {
    let now = (clock::timestamp_ms(clock) / 1000) as u32;

    assert!(is_valid_period(unix_time, now), EPastTimestamp);

    TimeLock::UnlockAt(unix_time)
}

/// Creates a new UnlockAtMs time lock that unlocks at a specific milliseconds based Unix timestamp.
public fun unlock_at_ms(unix_time_ms: u64, clock: &Clock): TimeLock {
    let now = clock::timestamp_ms(clock);

    assert!(is_valid_period_ms(unix_time_ms, now), EPastTimestamp);

    TimeLock::UnlockAtMs(unix_time_ms)
}

/// Creates a new UntilDestroyed lock that never unlocks until the locked object is destroyed.
public fun until_destroyed(): TimeLock {
    TimeLock::UntilDestroyed
}

/// Creates a new Infinite lock that never unlocks (permanent lock)
public fun infinite(): TimeLock {
    TimeLock::Infinite
}

/// Create a new lock that is not locked.
public fun none(): TimeLock {
    TimeLock::None
}

/// Checks if the provided lock time is an UntilDestroyed lock.
public fun is_until_destroyed(lock_time: &TimeLock): bool {
    match (lock_time) {
        TimeLock::UntilDestroyed => true,
        _ => false,
    }
}

/// Checks if the provided lock time is a UnlockAt lock.
public fun is_unlock_at(lock_time: &TimeLock): bool {
    match (lock_time) {
        TimeLock::UnlockAt(_) => true,
        _ => false,
    }
}

/// Checks if the provided lock time is a UnlockAt lock.
public fun is_unlock_at_ms(lock_time: &TimeLock): bool {
    match (lock_time) {
        TimeLock::UnlockAtMs(_) => true,
        _ => false,
    }
}

/// Checks if the provided lock time is a None lock.
public fun is_none(lock_time: &TimeLock): bool {
    match (lock_time) {
        TimeLock::None => true,
        _ => false,
    }
}

/// Checks if the provided lock time is a None lock.
public fun is_infinite(lock_time: &TimeLock): bool {
    match (lock_time) {
        TimeLock::Infinite => true,
        _ => false,
    }
}

/// Gets the unlock time from a TimeLock if it is a UnixTime lock.
public fun get_unlock_time(lock_time: &TimeLock): Option<u32> {
    match (lock_time) {
        TimeLock::UnlockAt(time) => option::some(*time),
        _ => option::none(),
    }
}

/// Gets the unlock time from a TimeLock if it is a UnlockAtMs lock.
public fun get_unlock_time_ms(lock_time: &TimeLock): Option<u64> {
    match (lock_time) {
        TimeLock::UnlockAtMs(time) => option::some(*time),
        _ => option::none(),
    }
}

/// Destroys a TimeLock if it's either unlocked or an UntilDestroyed lock.
public fun destroy(condition: TimeLock, clock: &Clock) {
    // The TimeLock is always destroyed, except of those cases where an assertion is raised
    match (condition) {
        TimeLock::UnlockAt(time) => {
            assert!(!(time > ((clock::timestamp_ms(clock) / 1000) as u32)), ETimelockNotExpired);
        },
        TimeLock::UnlockAtMs(time) => {
            assert!(!(time > clock::timestamp_ms(clock)), ETimelockNotExpired);
        },
        TimeLock::UntilDestroyed => {},
        TimeLock::None => {},
        TimeLock::Infinite => {
            assert!(false, ETimelockNotExpired);
        },
    }
}

/// Checks if a timelock condition is currently active (locked).
///
/// This function evaluates whether a given TimeLock instance is currently in a locked state
/// by comparing the current time with the lock's parameters. A lock is considered active if:
/// 1. For UnixTime locks: The current time hasn't reached the specified unlock time yet
/// 2. For UntilDestroyed: Always returns true as these locks never unlock until the locked object is destroyed
/// 3. For None: Always returns false as there is no lock
public fun is_timelocked(condition: &TimeLock, clock: &Clock): bool {
    match (condition) {
        TimeLock::UnlockAt(unix_time) => {
            *unix_time > ((clock::timestamp_ms(clock) / 1000) as u32)
        },
        TimeLock::UnlockAtMs(unix_time_ms) => {
            *unix_time_ms > clock::timestamp_ms(clock)
        },
        TimeLock::UntilDestroyed => true,
        TimeLock::None => false,
        TimeLock::Infinite => true,
    }
}

/// Check if a timelock condition is `UnlockAt`
public fun is_timelocked_unlock_at(lock_time: &TimeLock, clock: &Clock): bool {
    match (lock_time) {
        TimeLock::UnlockAt(time) => {
            *time > ((clock::timestamp_ms(clock) / 1000) as u32)
        },
        TimeLock::UnlockAtMs(time_ms) => {
            *time_ms > clock::timestamp_ms(clock)
        },
        _ => false,
    }
}

/// Validates that a specified unlock time is in the future.
public fun is_valid_period(unix_time: u32, current_time: u32): bool {
    unix_time > current_time
}

/// Validates that a specified unlock time is in the future (using milliseconds based timestamps).
public fun is_valid_period_ms(unix_time: u64, current_time: u64): bool {
    unix_time > current_time
}

#[test_only]
/// Test helper to delete a TimeLock for testing purposes, especially useful for Infinite locks.
public fun destroy_for_testing(lock: TimeLock) {
    match (lock) {
        TimeLock::UnlockAt(_time) => {},
        TimeLock::UnlockAtMs(_time_ms) => {},
        TimeLock::UntilDestroyed => {},
        TimeLock::None => {},
        TimeLock::Infinite => {},
    }
}
