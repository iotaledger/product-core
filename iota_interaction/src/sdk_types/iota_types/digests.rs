// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk_types::Digest;

pub type CheckpointDigest = Digest;
pub type CheckpointContentsDigest = Digest;
pub type CertificateDigest = Digest;
pub type SenderSignedDataDigest = Digest;
pub type TransactionDigest = Digest;
pub type TransactionEffectsDigest = Digest;
pub type TransactionEventsDigest = Digest;
pub type EffectsAuxDataDigest = Digest;
pub type ObjectDigest = Digest;
pub type ConsensusCommitDigest = Digest;
pub type MoveAuthenticatorDigest = Digest;
pub type MisbehaviorReportDigest = Digest;